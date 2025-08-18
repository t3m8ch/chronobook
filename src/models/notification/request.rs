use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationSettingsRequest {
    #[garde(skip)]
    pub branch_id: Uuid,

    #[garde(length(chars, min = 5, max = 8))]
    pub quiet_hours_start: Option<String>, // "22:00:00"

    #[garde(length(chars, min = 5, max = 8))]
    pub quiet_hours_end: Option<String>, // "08:00:00"

    #[garde(range(min = 1, max = 72))]
    pub smart_boundary_hours: Option<i32>,

    #[garde(range(min = 1, max = 24))]
    pub critical_threshold_hours: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateNotificationTemplateRequest {
    #[garde(skip)]
    pub branch_id: Uuid,

    #[garde(skip)]
    pub template_type: NotificationTemplateType,
    #[garde(skip)]
    pub method: NotifyMethod,

    #[garde(length(chars, min = 1, max = 2000))]
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotificationTemplateRequest {
    #[garde(skip)]
    pub template_id: Uuid,

    #[garde(skip)]
    pub template_type: Option<NotificationTemplateType>,
    #[garde(skip)]
    pub method: Option<NotifyMethod>,

    #[garde(length(chars, min = 1, max = 2000))]
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNotificationTemplateRequest {
    #[garde(skip)]
    pub template_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SendBulkNotificationRequest {
    #[garde(skip)]
    pub branch_id: Uuid,

    /// Стратегия выбора получателей
    #[serde(flatten)]
    #[garde(skip)]
    pub recipients: BulkRecipientStrategy,

    #[garde(skip)]
    pub method: NotifyMethod,

    #[garde(length(chars, min = 1, max = 2000))]
    pub message: String,

    /// Время отправки. Если не указано - отправляется немедленно
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[garde(skip)]
    pub scheduled_at: Option<String>, // ISO 8601
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum BulkRecipientStrategy {
    /// Конкретные клиенты по ID
    #[serde(rename_all = "camelCase")]
    Specific { customer_ids: Vec<Uuid> },

    /// Все клиенты филиала
    #[serde(rename_all = "camelCase")]
    AllCustomers,

    /// Клиенты с записями за период
    #[serde(rename_all = "camelCase")]
    RecentBookings { from_date: String, to_date: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum NotificationTemplateType {
    #[serde(rename = "booking_reminder")]
    BookingReminder,
    #[serde(rename = "booking_confirmed")]
    BookingConfirmed,
    #[serde(rename = "booking_cancelled")]
    BookingCancelled,
    #[serde(rename = "booking_rescheduled")]
    BookingRescheduled,
    #[serde(rename = "custom_message")]
    CustomMessage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum NotifyMethod {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "telegram")]
    Telegram,
}
