use chrono::NaiveDateTime;
use sqlx::{FromRow, types::BigDecimal};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Service {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub display_name: String,
    pub description: String,
    pub duration_minutes: i32,
    pub price: Option<BigDecimal>,
    pub master_id: Option<Uuid>,
}
