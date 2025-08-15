use poem_openapi::{ApiResponse, Object, OpenApi, payload::Json};

use crate::api::{error::ApiError, validators::PhoneValidator};

#[derive(Clone, Debug)]
pub struct AuthApi;

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct PhoneLoginRequest {
    #[oai(validator(custom = "PhoneValidator"))]
    pub phone: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct PhoneVerifyRequest {
    pub code: u32,
}

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

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct TelegramVerifyHash {
    pub hash: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct AccessToken {
    pub access_token: String,
}

#[OpenApi(prefix_path = "/auth")]
impl AuthApi {
    #[tracing::instrument]
    #[oai(path = "/login/phone", method = "post")]
    async fn login_via_phone(&self, Json(request): Json<PhoneLoginRequest>) -> PhoneLoginResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/verify/phone", method = "post")]
    async fn verify_phone(&self, Json(request): Json<PhoneVerifyRequest>) -> PhoneVerifyResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/login/telegram", method = "post")]
    async fn login_via_telegram(&self) -> TelegramLoginResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/verify/telegram", method = "post")]
    async fn verify_telegram(&self) -> TelegramVerifyResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/refresh", method = "post")]
    async fn refresh(&self) -> RefreshResponse {
        todo!()
    }
}
