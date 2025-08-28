use std::sync::Arc;

use async_trait::async_trait;
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use rand::{Rng, RngCore, rngs::OsRng};
use sha2::Sha256;

use crate::{
    models::auth::{
        request::{
            CreateProfileRequest, PhoneLoginRequest, PhoneVerifyRequest, TelegramAuthRequest,
            UpdateProfileRequest,
        },
        response::{PhoneLoginOk, TelegramVerifyHash, UserProfileResponse},
    },
    repositories::auth::AuthRepository,
    services::{
        auth::AuthService,
        errors::AuthServiceError,
        jwt::JwtManager,
        providers::{SmsProvider, TelegramProvider},
    },
};

pub struct AuthServiceImpl {
    auth_repo: Arc<dyn AuthRepository>,
    sms_provider: Arc<dyn SmsProvider>,
    telegram_provider: Arc<dyn TelegramProvider>,
    jwt_manager: Arc<JwtManager>,
    telegram_hash_secret: String,
}

impl AuthServiceImpl {
    pub fn new(
        auth_repo: Arc<dyn AuthRepository>,
        sms_provider: Arc<dyn SmsProvider>,
        telegram_provider: Arc<dyn TelegramProvider>,
        jwt_manager: Arc<JwtManager>,
        telegram_hash_secret: String,
    ) -> Self {
        Self {
            auth_repo,
            sms_provider,
            telegram_provider,
            jwt_manager,
            telegram_hash_secret,
        }
    }

    fn generate_verification_code() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(100000..=999999)
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn login_phone(
        &self,
        request: &PhoneLoginRequest,
    ) -> Result<PhoneLoginOk, AuthServiceError> {
        // Find or create user
        let user = match self.auth_repo.find_user_by_phone(&request.phone).await? {
            Some(user) => user,
            None => {
                self.auth_repo
                    .create_user(Some(request.phone.clone()), None)
                    .await?
            }
        };

        // Generate and save verification code
        let code = Self::generate_verification_code();
        self.auth_repo
            .create_phone_verify_code(user.id, code as i32)
            .await?;

        // Send SMS
        self.sms_provider
            .send_verification_code(&request.phone, code)
            .await
            .map_err(AuthServiceError::SmsSendError)?;

        Ok(PhoneLoginOk {
            message: "Verification code sent".to_string(),
        })
    }

    async fn verify_phone(
        &self,
        request: &PhoneVerifyRequest,
    ) -> Result<(String, String), AuthServiceError> {
        // Find user
        let user = self
            .auth_repo
            .find_user_by_phone(&request.phone)
            .await?
            .ok_or(AuthServiceError::UserNotFound)?;

        // Verify code
        let verify_code = self
            .auth_repo
            .find_valid_phone_verify_code(user.id, request.code as i32)
            .await?
            .ok_or(AuthServiceError::InvalidVerificationCode)?;

        // Mark code as used
        self.auth_repo
            .mark_phone_verify_code_used(verify_code.id)
            .await?;

        // Update user as verified if not already
        if user.phone_verified_at.is_none() {
            self.auth_repo.update_user_phone_verified(user.id).await?;
        }

        // Generate tokens for phone verification (no specific organization context yet)
        let user_types = vec![];

        let access_token = self
            .jwt_manager
            .generate_access_token(user.id, None, user_types.clone())
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        let refresh_token = self
            .jwt_manager
            .generate_refresh_token(user.id, None, user_types)
            .await
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        Ok((access_token, refresh_token))
    }

    async fn login_telegram(&self) -> Result<TelegramVerifyHash, AuthServiceError> {
        // Generate random salt (16 bytes for good entropy)
        let mut salt = vec![0u8; 16];
        OsRng.fill_bytes(&mut salt);

        // Create HMAC-SHA256 with secret key
        let mut mac = Hmac::<Sha256>::new_from_slice(self.telegram_hash_secret.as_bytes())
            .map_err(|e| {
                AuthServiceError::InternalError(format!("HMAC initialization failed: {}", e))
            })?;

        // Add salt to HMAC
        mac.update(&salt);

        // Get the HMAC result
        let result = mac.finalize();
        let hmac_bytes = result.into_bytes();

        // Combine salt and HMAC (16 bytes salt + 32 bytes HMAC = 48 bytes)
        let mut combined = Vec::with_capacity(48);
        combined.extend_from_slice(&salt);
        combined.extend_from_slice(&hmac_bytes);

        // Save hash to database (without user_id for now)
        self.auth_repo
            .create_telegram_verify_hash(None, combined.clone())
            .await?;

        // Encode to base64url (48 bytes -> 64 chars in base64url without padding)
        let hash_string = BASE64_URL_SAFE_NO_PAD.encode(&combined);

        Ok(TelegramVerifyHash { hash: hash_string })
    }

    async fn verify_telegram(
        &self,
        request: &TelegramAuthRequest,
    ) -> Result<(String, String), AuthServiceError> {
        // Decode the hash from base64url
        let hash_bytes = BASE64_URL_SAFE_NO_PAD
            .decode(&request.hash)
            .map_err(|e| AuthServiceError::InternalError(format!("Invalid hash format: {}", e)))?;

        // Find the hash in database
        let telegram_hash = self
            .auth_repo
            .find_valid_telegram_hash(&hash_bytes)
            .await?
            .ok_or(AuthServiceError::InvalidTelegramHash)?;

        // Check if the hash has been linked to a user (bot has processed it)
        let user_id = telegram_hash
            .user_id
            .ok_or(AuthServiceError::TelegramHashNotVerified)?;

        // Mark hash as used
        self.auth_repo
            .mark_telegram_hash_used(telegram_hash.id)
            .await?;

        // Get the user
        let user = self
            .auth_repo
            .find_user_by_id(user_id)
            .await?
            .ok_or(AuthServiceError::UserNotFoundById(user_id))?;

        // Generate tokens for telegram verification (no specific organization context yet)
        let user_types = vec![];

        let access_token = self
            .jwt_manager
            .generate_access_token(user.id, None, user_types.clone())
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        let refresh_token = self
            .jwt_manager
            .generate_refresh_token(user.id, None, user_types)
            .await
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        Ok((access_token, refresh_token))
    }

    async fn link_telegram_hash(
        &self,
        hash: &str,
        telegram_id: i64,
    ) -> Result<(), AuthServiceError> {
        // Decode the hash from base64url
        let hash_bytes = BASE64_URL_SAFE_NO_PAD
            .decode(hash)
            .map_err(|e| AuthServiceError::InternalError(format!("Invalid hash format: {}", e)))?;

        // Find the hash in database
        let telegram_hash = self
            .auth_repo
            .find_valid_telegram_hash(&hash_bytes)
            .await?
            .ok_or(AuthServiceError::InvalidTelegramHash)?;

        // Check if hash is already linked
        if telegram_hash.user_id.is_some() {
            return Err(AuthServiceError::TelegramHashAlreadyUsed);
        }

        // Find or create user by telegram_id
        let user = match self.auth_repo.find_user_by_telegram_id(telegram_id).await? {
            Some(user) => user,
            None => {
                // Create new user with telegram_id
                let new_user = self.auth_repo.create_user(None, Some(telegram_id)).await?;

                // Mark as verified immediately since they're coming from Telegram
                self.auth_repo
                    .update_user_telegram_verified(new_user.id)
                    .await?;

                new_user
            }
        };

        // Link hash with user
        self.auth_repo
            .update_telegram_hash_user(telegram_hash.id, user.id)
            .await?;

        Ok(())
    }

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(String, String), AuthServiceError> {
        // Verify refresh token
        let token_data = self
            .jwt_manager
            .verify_refresh_token(refresh_token)
            .await
            .map_err(|_| AuthServiceError::InvalidRefreshToken)?;

        // Check if user still exists
        let user = self
            .auth_repo
            .find_user_by_id(token_data.claims.sub)
            .await?
            .ok_or(AuthServiceError::UserNotFoundById(token_data.claims.sub))?;

        // Generate new tokens with existing user types
        let new_access_token = self
            .jwt_manager
            .generate_access_token(
                user.id,
                token_data.claims.org,
                token_data.claims.user_types.clone(),
            )
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        let new_refresh_token = self
            .jwt_manager
            .generate_refresh_token(user.id, token_data.claims.org, token_data.claims.user_types)
            .await
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        Ok((new_access_token, new_refresh_token))
    }

    async fn logout(&self, refresh_token: &str) -> Result<(), AuthServiceError> {
        self.jwt_manager
            .revoke_refresh_token(refresh_token)
            .await
            .map_err(|_| AuthServiceError::InvalidRefreshToken)
    }

    async fn logout_all(&self, refresh_token: &str) -> Result<(), AuthServiceError> {
        // First verify the token to get user ID
        let token_data = self
            .jwt_manager
            .verify_refresh_token(refresh_token)
            .await
            .map_err(|_| AuthServiceError::InvalidRefreshToken)?;

        self.jwt_manager
            .revoke_all_user_tokens(token_data.claims.sub)
            .await
            .map_err(|_| AuthServiceError::InvalidRefreshToken)
    }

    async fn create_profile(
        &self,
        user_id: uuid::Uuid,
        request: &CreateProfileRequest,
    ) -> Result<UserProfileResponse, AuthServiceError> {
        // Check if profile already exists
        if self.auth_repo.find_user_profile(user_id).await?.is_some() {
            return Err(AuthServiceError::ProfileAlreadyExists);
        }

        // Create new profile
        let profile = self
            .auth_repo
            .create_user_profile(
                user_id,
                &request.first_name,
                &request.last_name,
                request.patronymic.clone(),
            )
            .await?;

        Ok(UserProfileResponse {
            first_name: profile.first_name,
            last_name: profile.last_name,
            patronymic: profile.patronymic,
        })
    }

    async fn update_profile(
        &self,
        user_id: uuid::Uuid,
        request: &UpdateProfileRequest,
    ) -> Result<UserProfileResponse, AuthServiceError> {
        // Check if profile exists
        let existing_profile = self
            .auth_repo
            .find_user_profile(user_id)
            .await?
            .ok_or(AuthServiceError::ProfileNotFound)?;

        // Prepare updated values
        let first_name = request
            .first_name
            .as_ref()
            .unwrap_or(&existing_profile.first_name);
        let last_name = request
            .last_name
            .as_ref()
            .unwrap_or(&existing_profile.last_name);
        let patronymic = match &request.patronymic {
            Some(Some(value)) => Some(value.clone()),
            Some(None) => None,
            None => existing_profile.patronymic.clone(),
        };

        // Update profile
        let profile = self
            .auth_repo
            .update_user_profile(user_id, first_name, last_name, patronymic)
            .await?;

        Ok(UserProfileResponse {
            first_name: profile.first_name,
            last_name: profile.last_name,
            patronymic: profile.patronymic,
        })
    }

    async fn get_profile(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Option<UserProfileResponse>, AuthServiceError> {
        match self.auth_repo.find_user_profile(user_id).await? {
            Some(profile) => Ok(Some(UserProfileResponse {
                first_name: profile.first_name,
                last_name: profile.last_name,
                patronymic: profile.patronymic,
            })),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::auth::db::{PhoneVerifyCode, User},
        repositories::auth::MockAuthRepository,
        services::{
            jwt::UserType,
            providers::{MockSmsProvider, MockTelegramProvider},
        },
    };
    use chrono::Utc;
    use uuid::Uuid;

    // Helper function to create service with JWT manager only
    fn create_service_with_mocks(
        mock_repo: MockAuthRepository,
        mock_sms: MockSmsProvider,
        mock_telegram: MockTelegramProvider,
    ) -> AuthServiceImpl {
        use crate::repositories::token::MockTokenRepository;

        let mut mock_token_repository = MockTokenRepository::new();
        mock_token_repository
            .expect_whitelist_token()
            .returning(|_, _, _| Ok(()));
        mock_token_repository
            .expect_is_token_whitelisted()
            .returning(|_| Ok(true));

        AuthServiceImpl::new(
            Arc::new(mock_repo),
            Arc::new(mock_sms),
            Arc::new(mock_telegram),
            Arc::new(
                JwtManager::builder()
                    .access_secret("secret")
                    .refresh_secret("secret")
                    .token_repository(Arc::new(mock_token_repository))
                    .build(),
            ),
            "test_telegram_secret".to_string(),
        )
    }

    fn create_test_user() -> User {
        User {
            id: Uuid::now_v7(),
            phone_verified_at: None,
        }
    }

    #[tokio::test]
    async fn test_login_phone_success_new_user() {
        let phone = "+1234567890".to_string();

        // Setup mocks before creating service
        let mut mock_repo = MockAuthRepository::new();
        let mut mock_sms = MockSmsProvider::new();

        mock_repo
            .expect_find_user_by_phone()
            .with(mockall::predicate::eq(phone.clone()))
            .times(1)
            .returning(|_| Ok(None));

        mock_repo
            .expect_create_user()
            .times(1)
            .returning(move |_, _| Ok(create_test_user()));

        mock_repo
            .expect_create_phone_verify_code()
            .times(1)
            .returning(move |_, _| Ok(PhoneVerifyCode { id: Uuid::now_v7() }));

        mock_sms
            .expect_send_verification_code()
            .times(1)
            .returning(|_, _| Ok(()));

        // Create service with configured mocks
        let service = AuthServiceImpl::new(
            Arc::new(mock_repo),
            Arc::new(mock_sms),
            Arc::new(MockTelegramProvider::new()),
            Arc::new(
                JwtManager::builder()
                    .access_secret("secret")
                    .refresh_secret("secret")
                    .token_repository({
                        use crate::repositories::token::MockTokenRepository;
                        let mut mock_token_repository = MockTokenRepository::new();
                        mock_token_repository
                            .expect_whitelist_token()
                            .returning(|_, _, _| Ok(()));
                        mock_token_repository
                            .expect_is_token_whitelisted()
                            .returning(|_| Ok(true));
                        Arc::new(mock_token_repository)
                    })
                    .build(),
            ),
            "test_telegram_secret".to_string(),
        );

        let request = PhoneLoginRequest {
            phone: phone.clone(),
        };

        let result = service.login_phone(&request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().message, "Verification code sent");
    }

    #[tokio::test]
    async fn test_login_phone_user_not_found() {
        let phone = "+1234567890".to_string();
        let mut mock_repo = MockAuthRepository::new();
        let mut mock_sms = MockSmsProvider::new();

        mock_repo
            .expect_find_user_by_phone()
            .with(mockall::predicate::eq(phone.clone()))
            .times(1)
            .returning(|_| Ok(Some(create_test_user())));

        mock_repo
            .expect_create_phone_verify_code()
            .times(1)
            .returning(move |_, _| Ok(PhoneVerifyCode { id: Uuid::now_v7() }));

        mock_sms
            .expect_send_verification_code()
            .times(1)
            .returning(|_, _| Ok(()));

        let service = create_service_with_mocks(mock_repo, mock_sms, MockTelegramProvider::new());

        let request = PhoneLoginRequest {
            phone: phone.clone(),
        };

        let result = service.login_phone(&request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_phone_success() {
        let phone = "+1234567890".to_string();
        let user = create_test_user();
        let user_id = user.id;
        let code = 123456;

        let mut mock_repo = MockAuthRepository::new();

        let user_clone = user.clone();
        mock_repo
            .expect_find_user_by_phone()
            .with(mockall::predicate::eq(phone.clone()))
            .times(1)
            .returning(move |_| Ok(Some(user_clone.clone())));

        mock_repo
            .expect_find_valid_phone_verify_code()
            .with(
                mockall::predicate::eq(user_id),
                mockall::predicate::eq(code as i32),
            )
            .times(1)
            .returning(move |_, _| Ok(Some(PhoneVerifyCode { id: Uuid::now_v7() })));

        mock_repo
            .expect_mark_phone_verify_code_used()
            .times(1)
            .returning(|_| Ok(()));

        mock_repo
            .expect_update_user_phone_verified()
            .times(1)
            .returning(move |uid| {
                let mut verified_user = create_test_user();
                verified_user.id = uid;
                verified_user.phone_verified_at = Some(Utc::now().naive_utc());
                Ok(verified_user)
            });

        let service = create_service_with_mocks(
            mock_repo,
            MockSmsProvider::new(),
            MockTelegramProvider::new(),
        );

        let request = PhoneVerifyRequest {
            phone: phone.clone(),
            code,
        };

        let result = service.verify_phone(&request).await;
        assert!(result.is_ok());
        let (access_token, refresh_token) = result.unwrap();
        assert!(!access_token.is_empty());
        assert!(!refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_verify_phone_invalid_code() {
        let phone = "+1234567890".to_string();
        let user = create_test_user();

        let mut mock_repo = MockAuthRepository::new();

        let user_clone = user.clone();
        mock_repo
            .expect_find_user_by_phone()
            .times(1)
            .returning(move |_| Ok(Some(user_clone.clone())));

        mock_repo
            .expect_find_valid_phone_verify_code()
            .times(1)
            .returning(|_, _| Ok(None));

        let service = create_service_with_mocks(
            mock_repo,
            MockSmsProvider::new(),
            MockTelegramProvider::new(),
        );

        let request = PhoneVerifyRequest {
            phone: phone.clone(),
            code: 999999,
        };

        let result = service.verify_phone(&request).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AuthServiceError::InvalidVerificationCode
        ));
    }

    #[tokio::test]
    async fn test_refresh_token_success() {
        let user = create_test_user();
        let org_id = Uuid::now_v7();

        // Generate a valid refresh token
        let jwt_manager = JwtManager::builder()
            .access_secret("secret")
            .refresh_secret("secret")
            .token_repository({
                use crate::repositories::token::MockTokenRepository;
                let mut mock_token_repository = MockTokenRepository::new();
                mock_token_repository
                    .expect_whitelist_token()
                    .returning(|_, _, _| Ok(()));
                mock_token_repository
                    .expect_is_token_whitelisted()
                    .returning(|_| Ok(true));
                Arc::new(mock_token_repository)
            })
            .build();

        let user_types = vec![UserType::Customer {
            id: user.id,
            org_id,
        }];
        let refresh_token = jwt_manager
            .generate_refresh_token(user.id, Some(org_id), user_types)
            .await
            .unwrap();

        let mut mock_repo = MockAuthRepository::new();

        let user_clone = user.clone();
        mock_repo
            .expect_find_user_by_id()
            .with(mockall::predicate::eq(user.id))
            .times(1)
            .returning(move |_| Ok(Some(user_clone.clone())));

        let service = create_service_with_mocks(
            mock_repo,
            MockSmsProvider::new(),
            MockTelegramProvider::new(),
        );

        let result = service.refresh_token(&refresh_token).await;
        assert!(result.is_ok());
        let (new_access_token, new_refresh_token) = result.unwrap();
        assert!(!new_access_token.is_empty());
        assert!(!new_refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_refresh_token_invalid() {
        let service = create_service_with_mocks(
            MockAuthRepository::new(),
            MockSmsProvider::new(),
            MockTelegramProvider::new(),
        );

        let result = service.refresh_token("invalid_token").await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AuthServiceError::InvalidRefreshToken
        ));
    }
}
