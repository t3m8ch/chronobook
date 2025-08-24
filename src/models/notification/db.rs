use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub branch_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub quiet_hours_start: NaiveTime,
    pub quiet_hours_end: NaiveTime,
    pub smart_boundary_hours: i32,
    pub critical_threshold_hours: i32,
}

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub branch_id: Uuid,
    pub template_type: String,
    pub method: String, // This will be 'sms' or 'telegram'
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct ScheduledNotification {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub booking_id: Uuid,
    pub method: String, // This will be 'sms' or 'telegram'
    pub template_id: Uuid,
    pub scheduled_at: DateTime<Utc>,
    pub actual_send_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
    pub status: String, // This will be 'pending', 'sent', 'failed', 'cancelled'
    pub error_message: Option<String>,
}
