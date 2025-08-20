use bon::Builder;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::repositories::token::{TokenRepository, TokenRepositoryError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,         // User ID
    pub org: Option<Uuid>, // Organization ID for customers
    pub exp: i64,          // Expiration time
    pub iat: i64,          // Issued at
    pub jti: String,       // JWT ID for token tracking
    pub token_type: TokenType,
    pub user_types: Vec<UserType>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum UserType {
    Customer {
        id: Uuid,
        org_id: Uuid,
    },
    Employee {
        id: Uuid,
        roles: Vec<UserRole>,
        org_id: Uuid,
        master_branch_id: Option<Uuid>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum UserRole {
    Owner,
    Manager,
    Master,
}

#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("JWT encoding/decoding error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Token repository error: {0}")]
    TokenRepository(#[from] TokenRepositoryError),
    #[error("Invalid token type")]
    InvalidTokenType,
    #[error("Token not whitelisted")]
    TokenNotWhitelisted,
}

#[derive(Clone, Builder)]
#[builder(on(String, into))]
pub struct JwtManager {
    access_secret: String,

    refresh_secret: String,

    #[builder(default = Duration::minutes(15))]
    access_duration: Duration,

    #[builder(default = Duration::days(7))]
    refresh_duration: Duration,

    token_repository: Arc<dyn TokenRepository>,
}

impl JwtManager {
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        organization_id: Option<Uuid>,
        user_types: Vec<UserType>,
    ) -> Result<String, JwtError> {
        let now = Utc::now();
        let exp = now + self.access_duration;
        let jti = Uuid::now_v7().to_string();

        let claims = Claims {
            sub: user_id,
            org: organization_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti,
            token_type: TokenType::Access,
            user_types,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.access_secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub async fn generate_refresh_token(
        &self,
        user_id: Uuid,
        organization_id: Option<Uuid>,
        user_types: Vec<UserType>,
    ) -> Result<String, JwtError> {
        let now = Utc::now();
        let exp = now + self.refresh_duration;
        let jti = Uuid::now_v7().to_string();

        let claims = Claims {
            sub: user_id,
            org: organization_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: jti.clone(),
            token_type: TokenType::Refresh,
            user_types,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.refresh_secret.as_bytes()),
        )?;

        // Add token to whitelist
        let ttl = std::time::Duration::from_secs(self.refresh_duration.num_seconds() as u64);
        self.token_repository
            .whitelist_token(&jti, user_id, ttl)
            .await?;

        Ok(token)
    }

    pub fn verify_access_token(&self, token: &str) -> Result<TokenData<Claims>, JwtError> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.access_secret.as_bytes()),
            &validation,
        )?;

        if token_data.claims.token_type != TokenType::Access {
            return Err(JwtError::InvalidTokenType);
        }

        Ok(token_data)
    }

    pub async fn verify_refresh_token(&self, token: &str) -> Result<TokenData<Claims>, JwtError> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.refresh_secret.as_bytes()),
            &validation,
        )?;

        if token_data.claims.token_type != TokenType::Refresh {
            return Err(JwtError::InvalidTokenType);
        }

        // Check if token is whitelisted
        let is_whitelisted = self
            .token_repository
            .is_token_whitelisted(&token_data.claims.jti)
            .await?;
        if !is_whitelisted {
            return Err(JwtError::TokenNotWhitelisted);
        }

        Ok(token_data)
    }

    /// Revoke a refresh token (logout)
    pub async fn revoke_refresh_token(&self, token: &str) -> Result<(), JwtError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.refresh_secret.as_bytes()),
            &Validation::default(),
        )?;

        if token_data.claims.token_type != TokenType::Refresh {
            return Err(JwtError::InvalidTokenType);
        }

        self.token_repository
            .remove_token(&token_data.claims.jti)
            .await?;
        Ok(())
    }

    /// Revoke all refresh tokens for a user (logout from all devices)
    pub async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<(), JwtError> {
        self.token_repository
            .remove_all_user_tokens(user_id)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::token::MockTokenRepository;

    fn create_mock_token_repository() -> Arc<MockTokenRepository> {
        let mut mock_repo = MockTokenRepository::new();

        // Setup default expectations for whitelist operations
        mock_repo
            .expect_whitelist_token()
            .returning(|_, _, _| Ok(()));

        mock_repo
            .expect_is_token_whitelisted()
            .returning(|_| Ok(true));

        Arc::new(mock_repo)
    }

    #[test]
    fn test_generate_and_verify_access_token() {
        let jwt_manager = JwtManager::builder()
            .access_secret("secret")
            .refresh_secret("secret")
            .token_repository(create_mock_token_repository())
            .build();
        let user_id = Uuid::now_v7();
        let org_id = Some(Uuid::now_v7());
        let user_types = vec![UserType::Customer {
            id: user_id,
            org_id: org_id.unwrap(),
        }];

        let token = jwt_manager
            .generate_access_token(user_id, org_id, user_types.clone())
            .expect("Failed to generate access token");

        let token_data = jwt_manager
            .verify_access_token(&token)
            .expect("Failed to verify access token");

        assert_eq!(token_data.claims.sub, user_id);
        assert_eq!(token_data.claims.org, org_id);
        assert_eq!(token_data.claims.token_type, TokenType::Access);
        assert_eq!(token_data.claims.user_types, user_types);
        assert!(!token_data.claims.jti.is_empty());
    }

    #[tokio::test]
    async fn test_generate_and_verify_refresh_token() {
        let jwt_manager = JwtManager::builder()
            .access_secret("secret")
            .refresh_secret("secret")
            .token_repository(create_mock_token_repository())
            .build();
        let user_id = Uuid::now_v7();
        let org_id = Some(Uuid::now_v7());
        let user_types = vec![UserType::Employee {
            id: user_id,
            roles: vec![UserRole::Manager],
            org_id: org_id.unwrap(),
            master_branch_id: None,
        }];

        let token = jwt_manager
            .generate_refresh_token(user_id, org_id, user_types.clone())
            .await
            .expect("Failed to generate refresh token");

        let token_data = jwt_manager
            .verify_refresh_token(&token)
            .await
            .expect("Failed to verify refresh token");

        assert_eq!(token_data.claims.sub, user_id);
        assert_eq!(token_data.claims.org, org_id);
        assert_eq!(token_data.claims.token_type, TokenType::Refresh);
        assert_eq!(token_data.claims.user_types, user_types);
        assert!(!token_data.claims.jti.is_empty());
    }

    #[tokio::test]
    async fn test_invalid_token_type() {
        let jwt_manager = JwtManager::builder()
            .access_secret("secret")
            .refresh_secret("secret")
            .token_repository(create_mock_token_repository())
            .build();

        let user_id = Uuid::now_v7();
        let org_id = Uuid::now_v7();
        let user_types = vec![UserType::Customer {
            id: user_id,
            org_id,
        }];

        let refresh_token = jwt_manager
            .generate_refresh_token(user_id, Some(org_id), user_types)
            .await
            .expect("Failed to generate refresh token");

        let result = jwt_manager.verify_access_token(&refresh_token);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_token_not_whitelisted() {
        let mut mock_repo = MockTokenRepository::new();

        mock_repo
            .expect_whitelist_token()
            .returning(|_, _, _| Ok(()));

        // Token will not be found in whitelist
        mock_repo
            .expect_is_token_whitelisted()
            .returning(|_| Ok(false));

        let jwt_manager = JwtManager::builder()
            .access_secret("secret")
            .refresh_secret("secret")
            .token_repository(Arc::new(mock_repo))
            .build();

        let user_id = Uuid::now_v7();
        let org_id = Some(Uuid::now_v7());
        let user_types = vec![UserType::Customer {
            id: user_id,
            org_id: org_id.unwrap(),
        }];

        let token = jwt_manager
            .generate_refresh_token(user_id, org_id, user_types)
            .await
            .expect("Failed to generate refresh token");

        let result = jwt_manager.verify_refresh_token(&token).await;
        assert!(matches!(result, Err(JwtError::TokenNotWhitelisted)));
    }
}
