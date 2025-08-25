use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Booking {
    pub id: Uuid,
    pub service_id: Uuid,
    pub master_id: Uuid,
    pub branch_id: Uuid,
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
}
