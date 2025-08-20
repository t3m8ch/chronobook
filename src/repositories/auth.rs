use async_trait::async_trait;
use chrono::{Duration, Utc};
use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::models::{
    auth::db::{Customer, Organization, PhoneVerifyCode, TelegramVerifyHash, User, UserProfile},
    employee::db::Employee,
};

#[mockall::automock]
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_user_by_phone(&self, phone: &str) -> Result<Option<User>>;
    async fn find_user_by_telegram_id(&self, telegram_id: i64) -> Result<Option<User>>;
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn create_user(&self, phone: Option<String>, telegram_id: Option<i64>) -> Result<User>;
    async fn update_user_phone_verified(&self, user_id: Uuid) -> Result<User>;
    async fn update_user_telegram_verified(&self, user_id: Uuid) -> Result<User>;

    async fn find_user_profile(&self, user_id: Uuid) -> Result<Option<UserProfile>>;
    async fn create_user_profile(
        &self,
        user_id: Uuid,
        first_name: &str,
        last_name: &str,
        patronymic: Option<String>,
    ) -> Result<UserProfile>;
    async fn update_user_profile(
        &self,
        user_id: Uuid,
        first_name: &str,
        last_name: &str,
        patronymic: Option<String>,
    ) -> Result<UserProfile>;

    async fn find_customer(&self, user_id: Uuid, organization_id: Uuid)
    -> Result<Option<Customer>>;
    async fn create_customer(&self, user_id: Uuid, organization_id: Uuid) -> Result<Customer>;

    async fn find_employee_by_user_and_org(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<Employee>>;

    async fn create_phone_verify_code(&self, user_id: Uuid, code: i32) -> Result<PhoneVerifyCode>;
    async fn find_valid_phone_verify_code(
        &self,
        user_id: Uuid,
        code: i32,
    ) -> Result<Option<PhoneVerifyCode>>;
    async fn mark_phone_verify_code_used(&self, id: Uuid) -> Result<()>;
    async fn delete_expired_phone_codes(&self) -> Result<u64>;

    async fn create_telegram_verify_hash(
        &self,
        user_id: Option<Uuid>,
        hash: Vec<u8>,
    ) -> Result<TelegramVerifyHash>;
    async fn find_valid_telegram_hash(&self, hash: &[u8]) -> Result<Option<TelegramVerifyHash>>;
    async fn update_telegram_hash_user(&self, hash_id: Uuid, user_id: Uuid) -> Result<()>;
    async fn mark_telegram_hash_used(&self, id: Uuid) -> Result<()>;
    async fn delete_expired_telegram_hashes(&self) -> Result<u64>;
}

pub struct PgAuthRepository {
    pool: PgPool,
}

impl PgAuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for PgAuthRepository {
    async fn find_user_by_phone(&self, phone: &str) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, created_at, updated_at, phone, telegram_id,
                   phone_verified_at, telegram_verified_at
            FROM users
            WHERE phone = $1
            "#,
            phone
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_user_by_telegram_id(&self, telegram_id: i64) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, created_at, updated_at, phone, telegram_id,
                   phone_verified_at, telegram_verified_at
            FROM users
            WHERE telegram_id = $1
            "#,
            telegram_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, created_at, updated_at, phone, telegram_id,
                   phone_verified_at, telegram_verified_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_user(&self, phone: Option<String>, telegram_id: Option<i64>) -> Result<User> {
        let now = Utc::now();
        let id = Uuid::now_v7();

        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, created_at, updated_at, phone, telegram_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, created_at, updated_at, phone, telegram_id,
                      phone_verified_at, telegram_verified_at
            "#,
            id,
            now.naive_utc(),
            now.naive_utc(),
            phone,
            telegram_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update_user_phone_verified(&self, user_id: Uuid) -> Result<User> {
        let now = Utc::now();

        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET phone_verified_at = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, created_at, updated_at, phone, telegram_id,
                      phone_verified_at, telegram_verified_at
            "#,
            now.naive_utc(),
            now.naive_utc(),
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update_user_telegram_verified(&self, user_id: Uuid) -> Result<User> {
        let now = Utc::now();

        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET telegram_verified_at = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, created_at, updated_at, phone, telegram_id,
                      phone_verified_at, telegram_verified_at
            "#,
            now.naive_utc(),
            now.naive_utc(),
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_user_profile(&self, user_id: Uuid) -> Result<Option<UserProfile>> {
        sqlx::query_as!(
            UserProfile,
            r#"
            SELECT user_id, created_at, updated_at, first_name, last_name, patronymic
            FROM user_profiles
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_user_profile(
        &self,
        user_id: Uuid,
        first_name: &str,
        last_name: &str,
        patronymic: Option<String>,
    ) -> Result<UserProfile> {
        let now = Utc::now();

        sqlx::query_as!(
            UserProfile,
            r#"
            INSERT INTO user_profiles (user_id, created_at, updated_at, first_name, last_name, patronymic)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING user_id, created_at, updated_at, first_name, last_name, patronymic
            "#,
            user_id,
            now.naive_utc(),
            now.naive_utc(),
            first_name,
            last_name,
            patronymic
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update_user_profile(
        &self,
        user_id: Uuid,
        first_name: &str,
        last_name: &str,
        patronymic: Option<String>,
    ) -> Result<UserProfile> {
        let now = Utc::now();

        sqlx::query_as!(
            UserProfile,
            r#"
            UPDATE user_profiles
            SET first_name = $1, last_name = $2, patronymic = $3, updated_at = $4
            WHERE user_id = $5
            RETURNING user_id, created_at, updated_at, first_name, last_name, patronymic
            "#,
            first_name,
            last_name,
            patronymic,
            now.naive_utc(),
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_customer(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<Customer>> {
        sqlx::query_as!(
            Customer,
            r#"
            SELECT id, created_at, updated_at, organization_id, user_id
            FROM customers
            WHERE user_id = $1 AND organization_id = $2
            "#,
            user_id,
            organization_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_customer(&self, user_id: Uuid, organization_id: Uuid) -> Result<Customer> {
        let now = Utc::now();
        let id = Uuid::now_v7();

        sqlx::query_as!(
            Customer,
            r#"
            INSERT INTO customers (id, created_at, updated_at, organization_id, user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, created_at, updated_at, organization_id, user_id
            "#,
            id,
            now.naive_utc(),
            now.naive_utc(),
            organization_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_employee_by_user_and_org(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<Employee>> {
        sqlx::query_as!(
            Employee,
            r#"
            SELECT id, created_at, updated_at, contact_phone, contact_email, contact_telegram,
                   is_owner, is_manager, is_master, organization_id, manager_branch_id, user_id
            FROM employees
            WHERE user_id = $1 AND organization_id = $2
            "#,
            user_id,
            organization_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_phone_verify_code(&self, user_id: Uuid, code: i32) -> Result<PhoneVerifyCode> {
        let now = Utc::now();
        let id = Uuid::now_v7();
        let expire_at = (now + Duration::minutes(5)).naive_utc();

        sqlx::query_as!(
            PhoneVerifyCode,
            r#"
            INSERT INTO phone_verify_codes (id, created_at, code, expire_at, used, user_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, created_at, code, expire_at, used, user_id
            "#,
            id,
            now.naive_utc(),
            code,
            expire_at,
            false,
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_valid_phone_verify_code(
        &self,
        user_id: Uuid,
        code: i32,
    ) -> Result<Option<PhoneVerifyCode>> {
        let now = Utc::now();

        sqlx::query_as!(
            PhoneVerifyCode,
            r#"
            SELECT id, created_at, code, expire_at, used, user_id
            FROM phone_verify_codes
            WHERE user_id = $1 AND code = $2 AND used = false AND expire_at > $3
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id,
            code,
            now.naive_utc()
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn mark_phone_verify_code_used(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE phone_verify_codes
            SET used = true
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_expired_phone_codes(&self) -> Result<u64> {
        let now = Utc::now();

        let result = sqlx::query!(
            r#"
            DELETE FROM phone_verify_codes
            WHERE expire_at < $1 OR used = true
            "#,
            now.naive_utc()
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    async fn create_telegram_verify_hash(
        &self,
        user_id: Option<Uuid>,
        hash: Vec<u8>,
    ) -> Result<TelegramVerifyHash> {
        let now = Utc::now();
        let id = Uuid::now_v7();
        let expire_at = (now + Duration::minutes(5)).naive_utc();

        sqlx::query_as!(
            TelegramVerifyHash,
            r#"
            INSERT INTO telegram_verify_hashes (id, created_at, hash, expire_at, used, user_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, created_at, hash, expire_at, used, user_id
            "#,
            id,
            now.naive_utc(),
            hash,
            expire_at,
            false,
            user_id as Option<Uuid>
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn find_valid_telegram_hash(&self, hash: &[u8]) -> Result<Option<TelegramVerifyHash>> {
        let now = Utc::now();

        sqlx::query_as!(
            TelegramVerifyHash,
            r#"
            SELECT id, created_at, hash, expire_at, used, user_id
            FROM telegram_verify_hashes
            WHERE hash = $1 AND used = false AND expire_at > $2
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            hash,
            now.naive_utc()
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn update_telegram_hash_user(&self, hash_id: Uuid, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE telegram_verify_hashes
            SET user_id = $1
            WHERE id = $2
            "#,
            user_id,
            hash_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn mark_telegram_hash_used(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE telegram_verify_hashes
            SET used = true
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_expired_telegram_hashes(&self) -> Result<u64> {
        let now = Utc::now();

        let result = sqlx::query!(
            r#"
            DELETE FROM telegram_verify_hashes
            WHERE expire_at < $1 OR used = true
            "#,
            now.naive_utc()
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
