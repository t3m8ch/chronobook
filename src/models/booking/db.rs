use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Booking {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub customer_id: Uuid,
    pub service_id: Uuid,
    pub master_id: Uuid,
    pub branch_id: Uuid,
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
    pub notify_methods: Vec<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct BookingWithDetails {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub customer_id: Uuid,
    pub service_id: Uuid,
    pub master_id: Uuid,
    pub branch_id: Uuid,
    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,
    pub notify_methods: Vec<String>,
    // Additional joined data
    pub service_name: String,
    pub master_first_name: String,
    pub master_last_name: String,
    pub branch_name: String,
}
