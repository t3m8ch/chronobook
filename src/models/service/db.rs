use sqlx::{FromRow, types::BigDecimal};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Service {
    pub id: Uuid,
    pub display_name: String,
    pub description: String,
    pub duration_minutes: i32,
    pub price: Option<BigDecimal>,
    pub organization_id: Option<Uuid>,
}
