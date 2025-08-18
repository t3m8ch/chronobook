use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub phone: Option<String>,
    pub telegram_id: Option<i64>,
    pub phone_verified_at: Option<NaiveDateTime>,
    pub telegram_verified_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub organization_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub display_name: String,
    pub description: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct PhoneVerifyCode {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub code: i32,
    pub expire_at: NaiveDateTime,
    pub used: bool,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, FromRow)]
pub struct TelegramVerifyHash {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub hash: Vec<u8>,
    pub expire_at: NaiveDateTime,
    pub used: bool,
    pub user_id: Uuid,
}
