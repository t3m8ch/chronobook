use bon::Builder;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,         // User ID
    pub org: Option<Uuid>, // Organization ID for customers
    pub exp: i64,          // Expiration time
    pub iat: i64,          // Issued at
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

#[derive(Debug, Clone, Builder)]
#[builder(on(String, into))]
pub struct JwtManager {
    access_secret: String,

    refresh_secret: String,

    #[builder(default = Duration::minutes(15))]
    access_duration: Duration,

    #[builder(default = Duration::days(7))]
    refresh_duration: Duration,
}

impl JwtManager {
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        organization_id: Option<Uuid>,
        user_types: Vec<UserType>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + self.access_duration;

        let claims = Claims {
            sub: user_id,
            org: organization_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: TokenType::Access,
            user_types,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.access_secret.as_bytes()),
        )
    }

    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        organization_id: Option<Uuid>,
        user_types: Vec<UserType>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let exp = now + self.refresh_duration;

        let claims = Claims {
            sub: user_id,
            org: organization_id,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: TokenType::Refresh,
            user_types,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.refresh_secret.as_bytes()),
        )
    }

    pub fn verify_access_token(
        &self,
        token: &str,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.access_secret.as_bytes()),
            &validation,
        )?;

        if token_data.claims.token_type != TokenType::Access {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }

        Ok(token_data)
    }

    pub fn verify_refresh_token(
        &self,
        token: &str,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.refresh_secret.as_bytes()),
            &validation,
        )?;

        if token_data.claims.token_type != TokenType::Refresh {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }

        Ok(token_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_access_token() {
        let jwt_manager = JwtManager::builder()
            .access_secret("secret")
            .refresh_secret("secret")
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
    }

    #[test]
    fn test_generate_and_verify_refresh_token() {
        let jwt_manager = JwtManager::builder()
            .access_secret("secret")
            .refresh_secret("secret")
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
            .expect("Failed to generate refresh token");

        let token_data = jwt_manager
            .verify_refresh_token(&token)
            .expect("Failed to verify refresh token");

        assert_eq!(token_data.claims.sub, user_id);
        assert_eq!(token_data.claims.org, org_id);
        assert_eq!(token_data.claims.token_type, TokenType::Refresh);
        assert_eq!(token_data.claims.user_types, user_types);
    }

    #[test]
    fn test_invalid_token_type() {
        let jwt_manager = JwtManager::builder()
            .access_secret("secret")
            .refresh_secret("secret")
            .build();

        let user_id = Uuid::now_v7();
        let org_id = Uuid::now_v7();
        let user_types = vec![UserType::Customer {
            id: user_id,
            org_id,
        }];

        let refresh_token = jwt_manager
            .generate_refresh_token(user_id, Some(org_id), user_types)
            .expect("Failed to generate refresh token");

        let result = jwt_manager.verify_access_token(&refresh_token);
        assert!(result.is_err());
    }
}
