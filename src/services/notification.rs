use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    models::notification::{
        request::{BulkRecipientStrategy, NotificationTemplateType, NotifyMethod},
        response::{
            BulkNotificationResponse, NotificationSettingsResponse, NotificationTemplateResponse,
            ScheduledNotificationResponse,
        },
    },
    services::errors::ServiceError,
};

#[mockall::automock]
#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn get_notification_settings(
        &self,
        branch_id: Uuid,
    ) -> Result<NotificationSettingsResponse, ServiceError>;
    async fn update_notification_settings(
        &self,
        branch_id: Uuid,
        quiet_hours_start: Option<String>,
        quiet_hours_end: Option<String>,
        smart_boundary_hours: Option<i32>,
        critical_threshold_hours: Option<i32>,
    ) -> Result<NotificationSettingsResponse, ServiceError>;

    async fn get_notification_templates(
        &self,
        branch_id: Uuid,
    ) -> Result<Vec<NotificationTemplateResponse>, ServiceError>;
    async fn get_notification_template(
        &self,
        branch_id: Uuid,
        template_id: Uuid,
    ) -> Result<NotificationTemplateResponse, ServiceError>;
    async fn create_notification_template(
        &self,
        branch_id: Uuid,
        template_type: NotificationTemplateType,
        method: NotifyMethod,
        body: String,
    ) -> Result<NotificationTemplateResponse, ServiceError>;
    async fn update_notification_template(
        &self,
        template_id: Uuid,
        template_type: Option<NotificationTemplateType>,
        method: Option<NotifyMethod>,
        body: Option<String>,
    ) -> Result<NotificationTemplateResponse, ServiceError>;
    async fn delete_notification_template(&self, template_id: Uuid) -> Result<(), ServiceError>;

    async fn send_bulk_notification(
        &self,
        branch_id: Uuid,
        recipients: BulkRecipientStrategy,
        method: NotifyMethod,
        message: String,
        scheduled_at: Option<DateTime<Utc>>,
    ) -> Result<BulkNotificationResponse, ServiceError>;

    async fn get_scheduled_notifications(
        &self,
        branch_id: Uuid,
    ) -> Result<Vec<ScheduledNotificationResponse>, ServiceError>;
    async fn get_booking_notifications(
        &self,
        booking_id: Uuid,
    ) -> Result<Vec<ScheduledNotificationResponse>, ServiceError>;
}
