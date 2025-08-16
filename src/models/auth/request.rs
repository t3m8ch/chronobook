use crate::models::validation::validate_phone;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
#[garde(context(()))]
pub struct PhoneLoginRequest {
    #[garde(custom(validate_phone))]
    #[schema(example = "+1234567890")]
    pub phone: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
#[garde(context(()))]
pub struct PhoneVerifyRequest {
    #[garde(custom(validate_phone))]
    #[schema(example = "+1234567890")]
    pub phone: String,

    #[garde(range(min = 100000, max = 999999))]
    #[schema(example = 123456)]
    pub code: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
#[garde(context(()))]
pub struct UpdateProfileRequest {
    #[garde(length(min = 1, max = 100))]
    #[schema(example = "John")]
    pub first_name: String,

    #[garde(length(min = 1, max = 100))]
    #[schema(example = "Doe")]
    pub last_name: String,

    #[garde(length(min = 1, max = 100))]
    #[schema(example = "Smith")]
    pub patronymic: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TelegramAuthRequest {
    #[schema(example = 123456789)]
    pub telegram_id: i64,

    #[schema(example = "john_doe")]
    pub username: Option<String>,

    #[schema(example = "John")]
    pub first_name: String,

    #[schema(example = "Doe")]
    pub last_name: Option<String>,

    #[schema(example = "hash_value")]
    pub hash: String,
}
