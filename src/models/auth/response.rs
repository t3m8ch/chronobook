use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::models::error::ApiError;

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum PhoneLoginResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 429)]
    TooManyRequests(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum TelegramLoginResponse {
    #[oai(status = 200)]
    Ok(Json<TelegramVerifyHash>),

    #[oai(status = 429)]
    TooManyRequests(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum PhoneVerifyResponse {
    #[oai(status = 200)]
    Ok(Json<AccessToken>),

    #[oai(status = 400)]
    InvalidCode(Json<ApiError>),

    #[oai(status = 400)]
    CodeExpired(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum TelegramVerifyResponse {
    #[oai(status = 200)]
    Ok(Json<AccessToken>),

    #[oai(status = 400)]
    Unverified(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum RefreshResponse {
    #[oai(status = 200)]
    Ok(Json<AccessToken>),

    #[oai(status = 400)]
    InvalidToken(Json<ApiError>),

    #[oai(status = 400)]
    TokenExpired(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum UpdateProfileResponse {
    #[oai(status = 200)]
    Ok(Json<AccessToken>),

    #[oai(status = 400)]
    InvalidToken(Json<ApiError>),

    #[oai(status = 400)]
    TokenExpired(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct TelegramVerifyHash {
    pub hash: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct AccessToken {
    pub access_token: String,
}
