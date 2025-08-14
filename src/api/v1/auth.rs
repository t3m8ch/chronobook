use poem_openapi::{ApiResponse, Object, OpenApi, param::Path, payload::Json};

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
pub enum CustomerPhoneLoginResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 404)]
    NotFound(Json<ApiError>),

    #[oai(status = 429)]
    TooManyRequests(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum CustomerTelegramLoginResponse {
    #[oai(status = 200)]
    Ok(Json<TelegramVerifyHash>),

    #[oai(status = 404)]
    NotFound(Json<ApiError>),

    #[oai(status = 429)]
    TooManyRequests(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum EmployeePhoneLoginResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 429)]
    TooManyRequests(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum EmployeeTelegramLoginResponse {
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
    #[oai(path = "/customer/{organization_name}/login/phone", method = "post")]
    async fn customer_login_via_phone(
        &self,
        Path(organization_name): Path<String>,
        Json(request): Json<PhoneLoginRequest>,
    ) -> CustomerPhoneLoginResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/customer/{organization_name}/verify/phone", method = "post")]
    async fn customer_verify_phone(
        &self,
        Path(organization_name): Path<String>,
        Json(request): Json<PhoneVerifyRequest>,
    ) -> PhoneVerifyResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/customer/{organization_name}/login/telegram", method = "post")]
    async fn customer_login_via_telegram(
        &self,
        Path(organization_name): Path<String>,
    ) -> CustomerTelegramLoginResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(
        path = "/customer/{organization_name}/verify/telegram",
        method = "post"
    )]
    async fn customer_verify_telegram(
        &self,
        Path(organization_name): Path<String>,
    ) -> TelegramVerifyResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/customer/{organization_name}/refresh", method = "post")]
    async fn customer_refresh(&self, Path(organization_name): Path<String>) -> RefreshResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/employee/login/phone", method = "post")]
    async fn employee_login_via_phone(
        &self,
        Json(request): Json<PhoneLoginRequest>,
    ) -> EmployeePhoneLoginResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/employee/verify/phone", method = "post")]
    async fn employee_verify_phone(
        &self,
        Json(request): Json<PhoneVerifyRequest>,
    ) -> PhoneVerifyResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/employee/login/telegram", method = "post")]
    async fn employee_login_via_telegram(&self) -> EmployeeTelegramLoginResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/employee/verify/telegram", method = "post")]
    async fn employee_verify_telegram(&self) -> TelegramVerifyResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/employee/refresh", method = "post")]
    async fn employee_refresh(&self) -> RefreshResponse {
        todo!()
    }
}
