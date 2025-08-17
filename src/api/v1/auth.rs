use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    models::{
        auth::{
            request::{
                PhoneLoginRequest, PhoneVerifyRequest, RefreshTokenRequest, TelegramAuthRequest,
            },
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
#[tracing::instrument]
pub async fn login_phone(
    Path(organization_name): Path<String>,
    State(_state): State<Arc<AppState>>,
    Json(request): Json<PhoneLoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    request.validate_ext()?;

    // TODO: Implement phone login logic
    Ok(Json(PhoneLoginOk {
        message: "Verification code sent".to_string(),
    }))
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
#[tracing::instrument]
pub async fn verify_phone(
    Path(organization_name): Path<String>,
    State(_state): State<Arc<AppState>>,
    Json(request): Json<PhoneVerifyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    request.validate_ext()?;

    // TODO: Implement phone verification logic
    Ok(Json(AccessToken {
        access_token: "mock_access_token".to_string(),
        refresh_token: "mock_refresh_token".to_string(),
    }))
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
#[tracing::instrument]
pub async fn login_telegram(
    Path(organization_name): Path<String>,
    State(_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement Telegram login logic
    Ok(Json(TelegramVerifyHash {
        hash: "mock_hash".to_string(),
    }))
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
#[tracing::instrument]
pub async fn verify_telegram(
    Path(organization_name): Path<String>,
    State(_state): State<Arc<AppState>>,
    Json(request): Json<TelegramAuthRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement Telegram verification logic
    Ok(Json(AccessToken {
        access_token: "mock_access_token".to_string(),
        refresh_token: "mock_refresh_token".to_string(),
    }))
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed", body = AccessToken),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "auth"
)]
#[tracing::instrument]
pub async fn refresh(
    Path(organization_name): Path<String>,
    State(_state): State<Arc<AppState>>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement token refresh logic
    Ok(Json(AccessToken {
        access_token: "new_access_token".to_string(),
        refresh_token: "new_refresh_token".to_string(),
    }))
}
