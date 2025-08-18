use async_trait::async_trait;
use mockall::automock;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum TokenStoreError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Token not found")]
    TokenNotFound,
}

#[automock]
#[async_trait]
pub trait TokenStore: Send + Sync {
    /// Add a refresh token to the whitelist with expiration
    async fn whitelist_token(
        &self,
        token_id: &str,
        user_id: Uuid,
        ttl: Duration,
    ) -> Result<(), TokenStoreError>;

    /// Check if a refresh token is whitelisted
    async fn is_token_whitelisted(&self, token_id: &str) -> Result<bool, TokenStoreError>;

    /// Remove a refresh token from whitelist (for logout)
    async fn remove_token(&self, token_id: &str) -> Result<(), TokenStoreError>;

    /// Remove all tokens for a user (for logout from all devices)
    async fn remove_all_user_tokens(&self, user_id: Uuid) -> Result<(), TokenStoreError>;

    /// Get user ID associated with a token
    async fn get_token_user(&self, token_id: &str) -> Result<Uuid, TokenStoreError>;
}

use redis::{AsyncCommands, Client};

pub struct RedisTokenStore {
    client: Client,
}

impl RedisTokenStore {
    pub fn new(redis_url: &str) -> Result<Self, TokenStoreError> {
        let client =
            Client::open(redis_url).map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        Ok(Self { client })
    }

    fn token_key(token_id: &str) -> String {
        format!("refresh_token:{}", token_id)
    }

    fn user_tokens_key(user_id: Uuid) -> String {
        format!("user_tokens:{}", user_id)
    }
}

#[async_trait]
impl TokenStore for RedisTokenStore {
    async fn whitelist_token(
        &self,
        token_id: &str,
        user_id: Uuid,
        ttl: Duration,
    ) -> Result<(), TokenStoreError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        let token_key = Self::token_key(token_id);
        let user_tokens_key = Self::user_tokens_key(user_id);

        let user_id_str = user_id.to_string();

        // Set token with user ID and TTL
        let _: () = conn
            .set_ex(&token_key, &user_id_str, ttl.as_secs())
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        // Add token to user's token set
        let _: u32 = conn
            .sadd(&user_tokens_key, token_id)
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        // Set TTL on user's token set (slightly longer than token TTL)
        let user_set_ttl = ttl.as_secs() + 3600; // +1 hour buffer
        let _: bool = conn
            .expire(&user_tokens_key, user_set_ttl as i64)
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        Ok(())
    }

    async fn is_token_whitelisted(&self, token_id: &str) -> Result<bool, TokenStoreError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        let token_key = Self::token_key(token_id);

        let exists: bool = conn
            .exists(&token_key)
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        Ok(exists)
    }

    async fn remove_token(&self, token_id: &str) -> Result<(), TokenStoreError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        let token_key = Self::token_key(token_id);

        // Get user ID before deleting
        let user_id_str: Option<String> = conn
            .get(&token_key)
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        if let Some(user_id_str) = user_id_str {
            let user_id = user_id_str
                .parse::<Uuid>()
                .map_err(|e| TokenStoreError::Serialization(e.to_string()))?;

            let user_tokens_key = Self::user_tokens_key(user_id);

            // Remove token from Redis
            let _: u32 = conn
                .del(&token_key)
                .await
                .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

            // Remove token from user's token set
            let _: u32 = conn
                .srem(&user_tokens_key, token_id)
                .await
                .map_err(|e| TokenStoreError::Connection(e.to_string()))?;
        }

        Ok(())
    }

    async fn remove_all_user_tokens(&self, user_id: Uuid) -> Result<(), TokenStoreError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        let user_tokens_key = Self::user_tokens_key(user_id);

        // Get all token IDs for the user
        let token_ids: Vec<String> = conn
            .smembers(&user_tokens_key)
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        // Delete all individual tokens
        for token_id in &token_ids {
            let token_key = Self::token_key(token_id);
            let _: u32 = conn
                .del(&token_key)
                .await
                .map_err(|e| TokenStoreError::Connection(e.to_string()))?;
        }

        // Delete user's token set
        let _: u32 = conn
            .del(&user_tokens_key)
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        Ok(())
    }

    async fn get_token_user(&self, token_id: &str) -> Result<Uuid, TokenStoreError> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        let token_key = Self::token_key(token_id);

        let user_id_str: Option<String> = conn
            .get(&token_key)
            .await
            .map_err(|e| TokenStoreError::Connection(e.to_string()))?;

        match user_id_str {
            Some(user_id_str) => user_id_str
                .parse::<Uuid>()
                .map_err(|e| TokenStoreError::Serialization(e.to_string())),
            None => Err(TokenStoreError::TokenNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_mock_token_store() {
        let mut mock_store = MockTokenStore::new();
        let token_id = "test_token_123";
        let user_id = Uuid::now_v7();
        let ttl = Duration::from_secs(3600);

        mock_store
            .expect_whitelist_token()
            .with(
                mockall::predicate::eq(token_id),
                mockall::predicate::eq(user_id),
                mockall::predicate::eq(ttl),
            )
            .times(1)
            .returning(|_, _, _| Ok(()));

        mock_store
            .expect_is_token_whitelisted()
            .with(mockall::predicate::eq(token_id))
            .times(1)
            .returning(|_| Ok(true));

        // Test the mock
        mock_store
            .whitelist_token(token_id, user_id, ttl)
            .await
            .unwrap();
        let is_whitelisted = mock_store.is_token_whitelisted(token_id).await.unwrap();
        assert!(is_whitelisted);
    }
}
