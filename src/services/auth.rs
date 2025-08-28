use async_trait::async_trait;

use crate::models::auth::{
    request::{
        CreateProfileRequest, PhoneLoginRequest, PhoneVerifyRequest, TelegramAuthRequest,
        UpdateProfileRequest,
    },
    response::{PhoneLoginOk, TelegramVerifyHash, UserProfileResponse},
};

use super::errors::AuthServiceError;

#[mockall::automock]
#[async_trait]
pub trait AuthService: Send + Sync {
    async fn login_phone(
        &self,
        request: &PhoneLoginRequest,
    ) -> Result<PhoneLoginOk, AuthServiceError>;

    async fn verify_phone(
        &self,
        request: &PhoneVerifyRequest,
    ) -> Result<(String, String), AuthServiceError>;

    async fn login_telegram(&self) -> Result<TelegramVerifyHash, AuthServiceError>;

    async fn verify_telegram(
        &self,
        request: &TelegramAuthRequest,
    ) -> Result<(String, String), AuthServiceError>;

    async fn link_telegram_hash(
        &self,
        hash: &str,
        telegram_id: i64,
    ) -> Result<(), AuthServiceError>;

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(String, String), AuthServiceError>;

    async fn logout(&self, refresh_token: &str) -> Result<(), AuthServiceError>;

    async fn logout_all(&self, refresh_token: &str) -> Result<(), AuthServiceError>;

    async fn create_profile(
        &self,
        user_id: uuid::Uuid,
        request: &CreateProfileRequest,
    ) -> Result<UserProfileResponse, AuthServiceError>;

    async fn update_profile(
        &self,
        user_id: uuid::Uuid,
        request: &UpdateProfileRequest,
    ) -> Result<UserProfileResponse, AuthServiceError>;

    async fn get_profile(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Option<UserProfileResponse>, AuthServiceError>;
}
