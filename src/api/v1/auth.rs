use axum::{Json, extract::State, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState, JwtCookieSettings,
    models::{
        auth::{
            request::{PhoneLoginRequest, PhoneVerifyRequest, TelegramAuthRequest},
            response::{AccessToken, PhoneLoginOk, TelegramVerifyHash},
        },
        error::ApiError,
        validation::ValidationExt,
    },
};

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(login_phone))
        .routes(routes!(verify_phone))
        .routes(routes!(login_telegram))
        .routes(routes!(verify_telegram))
        .routes(routes!(refresh))
        .routes(routes!(logout))
        .routes(routes!(logout_all))
}

#[utoipa::path(
    post,
    path = "/login/phone",
    request_body = PhoneLoginRequest,
    responses(
        (status = 200, description = "Verification code sent", body = PhoneLoginOk),
        (status = 429, description = "Too many requests", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(state))]
pub async fn login_phone(
    State(state): State<Arc<AppState>>,
    Json(request): Json<PhoneLoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    request.validate_ext()?;
    let result = state.auth_service.login_phone(&request).await?;
    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/verify/phone",
    request_body = PhoneVerifyRequest,
    responses(
        (status = 200, description = "Verification successful", body = AccessToken),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(state, jar))]
pub async fn verify_phone(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(request): Json<PhoneVerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    request.validate_ext()?;
    let (access_token, refresh_token) = state.auth_service.verify_phone(&request).await?;
    let refresh_cookie = token_cookie(&state.jwt_cookie_settings, refresh_token);
    Ok((jar.add(refresh_cookie), Json(AccessToken { access_token })))
}

#[utoipa::path(
    post,
    path = "/login/telegram",
    responses(
        (status = 200, description = "Hash for verification", body = TelegramVerifyHash),
        (status = 429, description = "Too many requests", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(state))]
pub async fn login_telegram(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let result = state.auth_service.login_telegram().await?;

    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/verify/telegram",
    request_body = TelegramAuthRequest,
    responses(
        (status = 200, description = "Verification successful", body = AccessToken),
        (status = 400, description = "Verification failed", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(state, jar))]
pub async fn verify_telegram(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(request): Json<TelegramAuthRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let (access_token, refresh_token) = state.auth_service.verify_telegram(&request).await?;
    let refresh_cookie = token_cookie(&state.jwt_cookie_settings, refresh_token);
    Ok((jar.add(refresh_cookie), Json(AccessToken { access_token })))
}

#[utoipa::path(
    post,
    path = "/refresh",
    responses(
        (status = 200, description = "Token refreshed", body = AccessToken),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(state, jar))]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    // Get refresh token from cookie
    let refresh_token = jar
        .get("refresh_token")
        .ok_or_else(|| ApiError::bad_request("Refresh token not found in cookies".to_string()))?
        .value();

    let (access_token, new_refresh_token) = state.auth_service.refresh_token(refresh_token).await?;
    let refresh_cookie = token_cookie(&state.jwt_cookie_settings, new_refresh_token);
    Ok((jar.add(refresh_cookie), Json(AccessToken { access_token })))
}

fn token_cookie<'a>(settings: &JwtCookieSettings, token: String) -> Cookie<'a> {
    Cookie::build((settings.cookie_name.clone(), token))
        .http_only(settings.http_only)
        .secure(settings.secure)
        .same_site(settings.same_site)
        .path(settings.path.clone())
        .max_age(chrono_duration_to_time(settings.max_age))
        .build()
}

#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(state, jar))]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    if let Some(refresh_token) = jar.get("refresh_token") {
        // Try to revoke the token, but don't fail if it's already invalid
        let _ = state.auth_service.logout(refresh_token.value()).await;
    }

    // Remove the cookie
    let expired_cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .path("/")
        .max_age(time::Duration::ZERO)
        .build();

    Ok((
        jar.add(expired_cookie),
        Json(serde_json::json!({"message": "Logged out successfully"})),
    ))
}

#[utoipa::path(
    post,
    path = "/logout/all",
    responses(
        (status = 200, description = "Logout from all devices successful"),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(state, jar))]
pub async fn logout_all(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    if let Some(refresh_token) = jar.get("refresh_token") {
        // Try to revoke all user tokens
        let _ = state.auth_service.logout_all(refresh_token.value()).await;
    }

    // Remove the cookie
    let expired_cookie = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .path("/")
        .max_age(time::Duration::ZERO)
        .build();

    Ok((
        jar.add(expired_cookie),
        Json(serde_json::json!({"message": "Logged out from all devices successfully"})),
    ))
}

fn chrono_duration_to_time(duration: chrono::Duration) -> time::Duration {
    time::Duration::new(
        duration.num_seconds() as i64,
        duration.num_nanoseconds().unwrap_or(0) as i32,
    )
}
