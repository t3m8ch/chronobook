use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl ApiError {
    pub fn new(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new("NOT_FOUND", message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new("BAD_REQUEST", message)
    }

    pub fn invalid_verification_code(message: impl Into<String>) -> Self {
        Self::new("INVALID_VERIFICATION_CODE", message)
    }

    pub fn verification_code_expired(message: impl Into<String>) -> Self {
        Self::new("VERIFICATION_CODE_EXPIRED", message)
    }

    pub fn invalid_telegram_hash(message: impl Into<String>) -> Self {
        Self::new("INVALID_TELEGRAM_HASH", message)
    }

    pub fn telegram_hash_not_verified(message: impl Into<String>) -> Self {
        Self::new("TELEGRAM_HASH_NOT_VERIFIED", message)
    }

    pub fn invalid_refresh_token(message: impl Into<String>) -> Self {
        Self::new("INVALID_REFRESH_TOKEN", message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new("UNAUTHORIZED", message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new("FORBIDDEN", message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new("CONFLICT", message)
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self::new("INTERNAL_SERVER_ERROR", message)
    }

    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::new("RATE_LIMIT_EXCEEDED", message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.error.as_str() {
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            "BAD_REQUEST" => StatusCode::BAD_REQUEST,
            "VALIDATION_ERROR" => StatusCode::BAD_REQUEST,
            "INVALID_VERIFICATION_CODE" => StatusCode::BAD_REQUEST,
            "VERIFICATION_CODE_EXPIRED" => StatusCode::BAD_REQUEST,
            "INVALID_TELEGRAM_HASH" => StatusCode::BAD_REQUEST,
            "TELEGRAM_HASH_NOT_VERIFIED" => StatusCode::BAD_REQUEST,
            "INVALID_REFRESH_TOKEN" => StatusCode::BAD_REQUEST,
            "UNAUTHORIZED" => StatusCode::UNAUTHORIZED,
            "FORBIDDEN" => StatusCode::FORBIDDEN,
            "CONFLICT" => StatusCode::CONFLICT,
            "RATE_LIMIT_EXCEEDED" => StatusCode::TOO_MANY_REQUESTS,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(self)).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::new("INTERNAL_ERROR", err.to_string())
    }
}
