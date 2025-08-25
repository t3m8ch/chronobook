use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub phone_verified_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserProfile {
    pub first_name: String,
    pub last_name: String,
    pub patronymic: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Customer {
    pub id: Uuid,
}

#[derive(Debug, Clone, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct PhoneVerifyCode {
    pub id: Uuid,
}

#[derive(Debug, Clone, FromRow)]
pub struct TelegramVerifyHash {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
}
