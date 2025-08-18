use super::request::{NotificationTemplateType, NotifyMethod};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettingsResponse {
    pub branch_id: Uuid,
    pub quiet_hours_start: String,
    pub quiet_hours_end: String,
    pub smart_boundary_hours: i32,
    pub critical_threshold_hours: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationTemplateResponse {
    pub id: Uuid,
    pub branch_id: Uuid,
    pub template_type: NotificationTemplateType,
    pub method: NotifyMethod,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BulkNotificationResponse {
    /// ID задачи массовой рассылки
    pub bulk_id: Uuid,

    /// Общее количество получателей
    pub total_recipients: usize,

    /// Количество успешно запланированных уведомлений
    pub scheduled_count: usize,

    /// Количество неудачных (например, у клиента нет телефона/телеграма)
    pub failed_count: usize,

    /// Время запланированной отправки
    pub scheduled_at: String,

    /// Список ошибок для неудачных получателей
    pub errors: Vec<BulkNotificationError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BulkNotificationError {
    pub customer_id: Uuid,
    pub error: String, // "No phone number", "No telegram", etc.
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledNotificationResponse {
    pub id: Uuid,
    pub booking_id: Uuid,
    pub method: NotifyMethod,
    pub scheduled_at: String,
    pub actual_send_at: String,
    pub sent_at: Option<String>,
    pub status: NotificationStatus,
    pub error_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum NotificationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
