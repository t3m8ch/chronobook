use async_trait::async_trait;
use rand::Rng;
use std::sync::Arc;

use crate::{
    models::auth::{
        request::{PhoneLoginRequest, PhoneVerifyRequest, TelegramAuthRequest},
        response::{PhoneLoginOk, TelegramVerifyHash},
    },
    repositories::auth::AuthRepository,
    services::jwt::JwtManager,
};

use super::{
    errors::AuthServiceError,
    providers::{SmsProvider, TelegramProvider},
};

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

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(String, String), AuthServiceError>;
}

pub struct AuthServiceImpl {
    auth_repo: Arc<dyn AuthRepository>,
    sms_provider: Arc<dyn SmsProvider>,
    telegram_provider: Arc<dyn TelegramProvider>,
    jwt_manager: Arc<JwtManager>,
}

impl AuthServiceImpl {
    pub fn new(
        auth_repo: Arc<dyn AuthRepository>,
        sms_provider: Arc<dyn SmsProvider>,
        telegram_provider: Arc<dyn TelegramProvider>,
        jwt_manager: Arc<JwtManager>,
    ) -> Self {
        Self {
            auth_repo,
            sms_provider,
            telegram_provider,
            jwt_manager,
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

        // Generate tokens without organization context
        let access_token = self
            .jwt_manager
            .generate_access_token(user.id, None)
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        let refresh_token = self
            .jwt_manager
            .generate_refresh_token(user.id, None)
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        Ok((access_token, refresh_token))
    }

    async fn login_telegram(&self) -> Result<TelegramVerifyHash, AuthServiceError> {
        // Generate random hash for verification
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.r#gen::<u8>()).collect();

        // Convert to hex string for display
        let hash_string = hex::encode(&random_bytes);

        Ok(TelegramVerifyHash { hash: hash_string })
    }

    async fn verify_telegram(
        &self,
        request: &TelegramAuthRequest,
    ) -> Result<(String, String), AuthServiceError> {
        // Verify telegram auth data
        let is_valid = self
            .telegram_provider
            .verify_auth_data(
                request.telegram_id,
                request.username.clone(),
                request.first_name.clone(),
                request.last_name.clone(),
                request.hash.clone(),
            )
            .await
            .map_err(AuthServiceError::TelegramError)?;

        if !is_valid {
            return Err(AuthServiceError::InvalidTelegramHash);
        }

        // Find or create user
        let user = match self
            .auth_repo
            .find_user_by_telegram_id(request.telegram_id)
            .await?
        {
            Some(user) => user,
            None => {
                let new_user = self
                    .auth_repo
                    .create_user(None, Some(request.telegram_id))
                    .await?;

                // Create user profile
                self.auth_repo
                    .create_user_profile(
                        new_user.id,
                        &request.first_name,
                        request.last_name.as_deref().unwrap_or(""),
                        None,
                    )
                    .await?;

                new_user
            }
        };

        // Update user as verified if not already
        if user.telegram_verified_at.is_none() {
            self.auth_repo
                .update_user_telegram_verified(user.id)
                .await?;
        }

        // Generate tokens without organization context
        let access_token = self
            .jwt_manager
            .generate_access_token(user.id, None)
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        let refresh_token = self
            .jwt_manager
            .generate_refresh_token(user.id, None)
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        Ok((access_token, refresh_token))
    }

    async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(String, String), AuthServiceError> {
        // Verify refresh token
        let token_data = self
            .jwt_manager
            .verify_refresh_token(refresh_token)
            .map_err(|_| AuthServiceError::InvalidRefreshToken)?;

        // Check if user still exists
        let user = self
            .auth_repo
            .find_user_by_id(token_data.claims.sub)
            .await?
            .ok_or(AuthServiceError::UserNotFoundById(token_data.claims.sub))?;

        // Generate new tokens
        let new_access_token = self
            .jwt_manager
            .generate_access_token(user.id, token_data.claims.org)
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        let new_refresh_token = self
            .jwt_manager
            .generate_refresh_token(user.id, token_data.claims.org)
            .map_err(|e| AuthServiceError::TokenGenerationError(e.to_string()))?;

        Ok((new_access_token, new_refresh_token))
    }
}

// Add hex module inline since we need it for hash encoding
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::auth::db::{PhoneVerifyCode, User},
        repositories::auth::MockAuthRepository,
        services::providers::{MockSmsProvider, MockTelegramProvider},
    };
    use chrono::Utc;
    use uuid::Uuid;

    // Helper function to create service with JWT manager only
    fn create_service_with_mocks(
        mock_repo: MockAuthRepository,
        mock_sms: MockSmsProvider,
        mock_telegram: MockTelegramProvider,
    ) -> AuthServiceImpl {
        AuthServiceImpl::new(
            Arc::new(mock_repo),
            Arc::new(mock_sms),
            Arc::new(mock_telegram),
            Arc::new(JwtManager::new()),
        )
    }

    fn create_test_user(phone: Option<String>, telegram_id: Option<i64>) -> User {
        User {
            id: Uuid::now_v7(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            phone,
            telegram_id,
            phone_verified_at: None,
            telegram_verified_at: None,
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
            .returning(move |p, _| Ok(create_test_user(p, None)));

        mock_repo
            .expect_create_phone_verify_code()
            .times(1)
            .returning(move |user_id, code| {
                Ok(PhoneVerifyCode {
                    id: Uuid::now_v7(),
                    created_at: Utc::now().naive_utc(),
                    code,
                    expire_at: (Utc::now() + chrono::Duration::minutes(5)).naive_utc(),
                    used: false,
                    user_id,
                })
            });

        mock_sms
            .expect_send_verification_code()
            .times(1)
            .returning(|_, _| Ok(()));

        // Create service with configured mocks
        let service = AuthServiceImpl::new(
            Arc::new(mock_repo),
            Arc::new(mock_sms),
            Arc::new(MockTelegramProvider::new()),
            Arc::new(JwtManager::new()),
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
            .returning(|_| {
                Ok(Some(create_test_user(
                    Some("+1234567890".to_string()),
                    None,
                )))
            });

        mock_repo
            .expect_create_phone_verify_code()
            .times(1)
            .returning(move |user_id, code| {
                Ok(PhoneVerifyCode {
                    id: Uuid::now_v7(),
                    created_at: Utc::now().naive_utc(),
                    code,
                    expire_at: (Utc::now() + chrono::Duration::minutes(5)).naive_utc(),
                    used: false,
                    user_id,
                })
            });

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
        let user = create_test_user(Some(phone.clone()), None);
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
            .returning(move |uid, c| {
                Ok(Some(PhoneVerifyCode {
                    id: Uuid::now_v7(),
                    created_at: Utc::now().naive_utc(),
                    code: c,
                    expire_at: (Utc::now() + chrono::Duration::minutes(5)).naive_utc(),
                    used: false,
                    user_id: uid,
                }))
            });

        mock_repo
            .expect_mark_phone_verify_code_used()
            .times(1)
            .returning(|_| Ok(()));

        let phone_clone = phone.clone();
        mock_repo
            .expect_update_user_phone_verified()
            .times(1)
            .returning(move |uid| {
                let mut verified_user = create_test_user(Some(phone_clone.clone()), None);
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
        let user = create_test_user(Some(phone.clone()), None);

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
        let user = create_test_user(Some("+1234567890".to_string()), None);
        let org_id = Uuid::now_v7();

        // Generate a valid refresh token
        let jwt_manager = JwtManager::new();
        let refresh_token = jwt_manager
            .generate_refresh_token(user.id, Some(org_id))
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
