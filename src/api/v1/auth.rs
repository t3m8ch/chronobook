use axum::{Json, extract::State, response::IntoResponse};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use std::sync::Arc;
use time::Duration;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
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

    // Create HTTP-only cookie for refresh token
    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(Duration::days(7))
        .build();

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

    // Create HTTP-only cookie for refresh token
    let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(Duration::days(7))
        .build();

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

    // Create new HTTP-only cookie for refresh token
    let refresh_cookie = Cookie::build(("refresh_token", new_refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(Duration::days(7))
        .build();

    Ok((jar.add(refresh_cookie), Json(AccessToken { access_token })))
}
