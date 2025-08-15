use crate::models::validation::PhoneValidator;
use poem_openapi::Object;

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct PhoneLoginRequest {
    #[oai(validator(custom = "PhoneValidator"))]
    pub phone: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct PhoneVerifyRequest {
    #[oai(validator(custom = "PhoneValidator"))]
    pub phone: String,

    pub code: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct UpdateProfileRequest {
    pub first_name: String,
    pub last_name: String,
    pub patronymic: Option<String>,
}
