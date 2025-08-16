use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
    routing::{Router, post},
};
use std::sync::Arc;

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

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        // Customer auth routes
        .route(
            "/customer/{organization_name}/login/phone",
            post(customer::login_phone),
        )
        .route(
            "/customer/{organization_name}/verify/phone",
            post(customer::verify_phone),
        )
        .route(
            "/customer/{organization_name}/login/telegram",
            post(customer::login_telegram),
        )
        .route(
            "/customer/{organization_name}/verify/telegram",
            post(customer::verify_telegram),
        )
        .route(
            "/customer/{organization_name}/refresh",
            post(customer::refresh),
        )
        // Employee auth routes
        .route("/employee/login/phone", post(employee::login_phone))
        .route("/employee/verify/phone", post(employee::verify_phone))
        .route("/employee/login/telegram", post(employee::login_telegram))
        .route("/employee/verify/telegram", post(employee::verify_telegram))
        .route("/employee/refresh", post(employee::refresh))
}

pub mod customer {
    use super::*;

    #[utoipa::path(
        post,
        path = "/api/v1/auth/customer/{organization_name}/login/phone",
        params(
            ("organization_name" = String, Path, description = "Organization name")
        ),
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
        path = "/api/v1/auth/customer/{organization_name}/verify/phone",
        params(
            ("organization_name" = String, Path, description = "Organization name")
        ),
        request_body = PhoneVerifyRequest,
        responses(
            (status = 200, description = "Verification successful", body = AccessToken),
            (status = 400, description = "Invalid code", body = ApiError),
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
        path = "/api/v1/auth/customer/{organization_name}/login/telegram",
        params(
            ("organization_name" = String, Path, description = "Organization name")
        ),
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
        path = "/api/v1/auth/customer/{organization_name}/verify/telegram",
        params(
            ("organization_name" = String, Path, description = "Organization name")
        ),
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
        path = "/api/v1/auth/customer/{organization_name}/refresh",
        params(
            ("organization_name" = String, Path, description = "Organization name")
        ),
        request_body = RefreshTokenRequest,
        responses(
            (status = 200, description = "Token refreshed", body = AccessToken),
            (status = 400, description = "Invalid token", body = ApiError),
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
}

pub mod employee {
    use super::*;

    #[utoipa::path(
        post,
        path = "/api/v1/auth/employee/login/phone",
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
        State(_state): State<Arc<AppState>>,
        Json(request): Json<PhoneLoginRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        request.validate_ext()?;

        // TODO: Implement employee phone login logic
        Ok(Json(PhoneLoginOk {
            message: "Verification code sent".to_string(),
        }))
    }

    #[utoipa::path(
        post,
        path = "/api/v1/auth/employee/verify/phone",
        request_body = PhoneVerifyRequest,
        responses(
            (status = 200, description = "Verification successful", body = AccessToken),
            (status = 400, description = "Invalid code", body = ApiError),
            (status = 500, description = "Internal server error", body = ApiError)
        ),
        tag = "auth"
    )]
    #[tracing::instrument]
    pub async fn verify_phone(
        State(_state): State<Arc<AppState>>,
        Json(request): Json<PhoneVerifyRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        request.validate_ext()?;

        // TODO: Implement employee phone verification logic
        Ok(Json(AccessToken {
            access_token: "mock_access_token".to_string(),
            refresh_token: "mock_refresh_token".to_string(),
        }))
    }

    #[utoipa::path(
        post,
        path = "/api/v1/auth/employee/login/telegram",
        responses(
            (status = 200, description = "Hash for verification", body = TelegramVerifyHash),
            (status = 429, description = "Too many requests", body = ApiError),
            (status = 500, description = "Internal server error", body = ApiError)
        ),
        tag = "auth"
    )]
    #[tracing::instrument]
    pub async fn login_telegram(
        State(_state): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, ApiError> {
        // TODO: Implement employee Telegram login logic
        Ok(Json(TelegramVerifyHash {
            hash: "mock_hash".to_string(),
        }))
    }

    #[utoipa::path(
        post,
        path = "/api/v1/auth/employee/verify/telegram",
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
        State(_state): State<Arc<AppState>>,
        Json(request): Json<TelegramAuthRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        // TODO: Implement employee Telegram verification logic
        Ok(Json(AccessToken {
            access_token: "mock_access_token".to_string(),
            refresh_token: "mock_refresh_token".to_string(),
        }))
    }

    #[utoipa::path(
        post,
        path = "/api/v1/auth/employee/refresh",
        request_body = RefreshTokenRequest,
        responses(
            (status = 200, description = "Token refreshed", body = AccessToken),
            (status = 400, description = "Invalid token", body = ApiError),
            (status = 500, description = "Internal server error", body = ApiError)
        ),
        tag = "auth"
    )]
    #[tracing::instrument]
    pub async fn refresh(
        State(_state): State<Arc<AppState>>,
        Json(request): Json<RefreshTokenRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        // TODO: Implement employee token refresh logic
        Ok(Json(AccessToken {
            access_token: "new_access_token".to_string(),
            refresh_token: "new_refresh_token".to_string(),
        }))
    }
}
