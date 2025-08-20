use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Branch {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub display_name: String,
    pub description: String,
    pub timezone: String,
    pub street: String,
    pub house_number: String,
    pub apartment_number: Option<String>,
    pub city: String,
    pub region: String,
    pub country: String,
    pub address_info: Option<String>,
    pub organization_id: Uuid,
}
