use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AuthServiceError {
    #[error("Organization '{0}' not found")]
    OrganizationNotFound(String),

    #[error("Invalid verification code")]
    InvalidVerificationCode,

    #[error("Verification code expired")]
    VerificationCodeExpired,

    #[error("Invalid telegram hash")]
    InvalidTelegramHash,

    #[error("User not found")]
    UserNotFound,

    #[error("User with ID {0} not found")]
    UserNotFoundById(Uuid),

    #[error("Invalid refresh token")]
    InvalidRefreshToken,

    #[error("Token generation failed: {0}")]
    TokenGenerationError(String),

    #[error("SMS sending failed: {0}")]
    SmsSendError(String),

    #[error("Telegram message failed: {0}")]
    TelegramError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Rate limit exceeded for phone {0}")]
    RateLimitExceeded(String),

    #[error("Invalid phone number format")]
    InvalidPhoneNumber,

    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<AuthServiceError> for crate::models::error::ApiError {
    fn from(err: AuthServiceError) -> Self {
        use crate::models::error::ApiError;

        match err {
            AuthServiceError::OrganizationNotFound(_) => ApiError::not_found(err.to_string()),
            AuthServiceError::InvalidVerificationCode => {
                ApiError::invalid_verification_code(err.to_string())
            }
            AuthServiceError::VerificationCodeExpired => {
                ApiError::verification_code_expired(err.to_string())
            }
            AuthServiceError::InvalidTelegramHash => {
                ApiError::invalid_telegram_hash(err.to_string())
            }
            AuthServiceError::InvalidRefreshToken => {
                ApiError::invalid_refresh_token(err.to_string())
            }
            AuthServiceError::InvalidPhoneNumber => ApiError::bad_request(err.to_string()),
            AuthServiceError::UserNotFound | AuthServiceError::UserNotFoundById(_) => {
                ApiError::not_found(err.to_string())
            }
            AuthServiceError::RateLimitExceeded(_) => ApiError::rate_limit(err.to_string()),
            AuthServiceError::DatabaseError(_)
            | AuthServiceError::TokenGenerationError(_)
            | AuthServiceError::SmsSendError(_)
            | AuthServiceError::TelegramError(_)
            | AuthServiceError::InternalError(_) => {
                ApiError::internal_server_error(err.to_string())
            }
        }
    }
}
