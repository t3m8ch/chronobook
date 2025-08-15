use poem_openapi::{OpenApi, payload::Json};

use crate::models::auth::{
    request::{PhoneLoginRequest, PhoneVerifyRequest, UpdateProfileRequest},
    response::{
        PhoneLoginResponse, PhoneVerifyResponse, RefreshResponse, TelegramLoginResponse,
        TelegramVerifyResponse, UpdateProfileResponse,
    },
};

#[derive(Clone, Debug)]
pub struct AuthApi;

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

    #[tracing::instrument]
    #[oai(path = "/profile", method = "put")]
    async fn update_profile(
        &self,
        Json(request): Json<UpdateProfileRequest>,
    ) -> UpdateProfileResponse {
        todo!()
    }
}
