use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub error: ErrorType,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorType {
    NotFound,
    BadRequest,
    InvalidVerificationCode,
    VerificationCodeExpired,
    InvalidTelegramHash,
    TelegramHashNotVerified,
    InvalidRefreshToken,
    Unauthorized,
    Forbidden,
    Conflict,
    InternalServer,
    RateLimitExceeded,
    Validation,
    NotImplemented,
}

impl ApiError {
    pub fn new(error_type: ErrorType, message: impl Into<String>) -> Self {
        Self {
            error: error_type,
            message: message.into(),
            details: None,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorType::NotFound, message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(ErrorType::BadRequest, message)
    }

    pub fn invalid_verification_code(message: impl Into<String>) -> Self {
        Self::new(ErrorType::InvalidVerificationCode, message)
    }

    pub fn verification_code_expired(message: impl Into<String>) -> Self {
        Self::new(ErrorType::VerificationCodeExpired, message)
    }

    pub fn invalid_telegram_hash(message: impl Into<String>) -> Self {
        Self::new(ErrorType::InvalidTelegramHash, message)
    }

    pub fn telegram_hash_not_verified(message: impl Into<String>) -> Self {
        Self::new(ErrorType::TelegramHashNotVerified, message)
    }

    pub fn invalid_refresh_token(message: impl Into<String>) -> Self {
        Self::new(ErrorType::InvalidRefreshToken, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(ErrorType::Unauthorized, message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(ErrorType::Forbidden, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorType::Conflict, message)
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self::new(ErrorType::InternalServer, message)
    }

    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::new(ErrorType::RateLimitExceeded, message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.error {
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::BadRequest => StatusCode::BAD_REQUEST,
            ErrorType::Validation => StatusCode::BAD_REQUEST,
            ErrorType::InvalidVerificationCode => StatusCode::BAD_REQUEST,
            ErrorType::VerificationCodeExpired => StatusCode::BAD_REQUEST,
            ErrorType::InvalidTelegramHash => StatusCode::BAD_REQUEST,
            ErrorType::TelegramHashNotVerified => StatusCode::BAD_REQUEST,
            ErrorType::InvalidRefreshToken => StatusCode::BAD_REQUEST,
            ErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorType::Forbidden => StatusCode::FORBIDDEN,
            ErrorType::Conflict => StatusCode::CONFLICT,
            ErrorType::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
            ErrorType::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::NotImplemented => StatusCode::NOT_IMPLEMENTED,
        };

        (status, Json(self)).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::new(ErrorType::InternalServer, err.to_string())
    }
}
