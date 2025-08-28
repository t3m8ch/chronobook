use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, NaiveTime, Utc};
use uuid::Uuid;

use crate::{
    models::notification::{
        BulkNotificationError, BulkNotificationResponse, BulkRecipientStrategy,
        NotificationSettingsResponse, NotificationStatus, NotificationTemplateResponse,
        NotificationTemplateType, NotifyMethod, ScheduledNotificationResponse,
        db::{NotificationSettings, NotificationTemplate, ScheduledNotification},
    },
    repositories::notification::NotificationRepository,
    services::{
        errors::ServiceError,
        notification::NotificationService,
        providers::{SmsProvider, TelegramProvider},
    },
};

pub struct NotificationServiceImpl {
    notification_repo: Arc<dyn NotificationRepository>,
    sms_provider: Arc<dyn SmsProvider>,
    telegram_provider: Arc<dyn TelegramProvider>,
}

impl NotificationServiceImpl {
    pub fn new(
        notification_repo: Arc<dyn NotificationRepository>,
        sms_provider: Arc<dyn SmsProvider>,
        telegram_provider: Arc<dyn TelegramProvider>,
    ) -> Self {
        Self {
            notification_repo,
            sms_provider,
            telegram_provider,
        }
    }
}

#[async_trait]
impl NotificationService for NotificationServiceImpl {
    async fn get_notification_settings(
        &self,
        branch_id: Uuid,
    ) -> Result<NotificationSettingsResponse, ServiceError> {
        let settings = match self
            .notification_repo
            .get_notification_settings(branch_id)
            .await
        {
            Ok(Some(settings)) => settings,
            Ok(None) => {
                // Create default settings if they don't exist
                self.notification_repo
                    .create_default_notification_settings(branch_id)
                    .await
                    .map_err(ServiceError::DatabaseError)?
            }
            Err(e) => return Err(ServiceError::DatabaseError(e)),
        };

        Ok(settings_to_response(settings))
    }

    async fn update_notification_settings(
        &self,
        branch_id: Uuid,
        quiet_hours_start: Option<String>,
        quiet_hours_end: Option<String>,
        smart_boundary_hours: Option<i32>,
        critical_threshold_hours: Option<i32>,
    ) -> Result<NotificationSettingsResponse, ServiceError> {
        // Parse time strings to NaiveTime if provided
        let start_time = match quiet_hours_start {
            Some(time_str) => Some(NaiveTime::parse_from_str(&time_str, "%H:%M:%S").map_err(
                |_| {
                    ServiceError::ValidationError(
                        "Invalid time format for quiet_hours_start".to_string(),
                    )
                },
            )?),
            None => None,
        };

        let end_time = match quiet_hours_end {
            Some(time_str) => Some(NaiveTime::parse_from_str(&time_str, "%H:%M:%S").map_err(
                |_| {
                    ServiceError::ValidationError(
                        "Invalid time format for quiet_hours_end".to_string(),
                    )
                },
            )?),
            None => None,
        };

        let settings = self
            .notification_repo
            .update_notification_settings(
                branch_id,
                start_time,
                end_time,
                smart_boundary_hours,
                critical_threshold_hours,
            )
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(settings_to_response(settings))
    }

    async fn get_notification_templates(
        &self,
        branch_id: Uuid,
    ) -> Result<Vec<NotificationTemplateResponse>, ServiceError> {
        let templates = self
            .notification_repo
            .get_notification_templates(branch_id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(templates.into_iter().map(template_to_response).collect())
    }

    async fn get_notification_template(
        &self,
        branch_id: Uuid,
        template_id: Uuid,
    ) -> Result<NotificationTemplateResponse, ServiceError> {
        let template = self
            .notification_repo
            .get_notification_template(branch_id, template_id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound(
                "Notification template not found".to_string(),
            ))?;

        Ok(template_to_response(template))
    }

    async fn create_notification_template(
        &self,
        branch_id: Uuid,
        template_type: NotificationTemplateType,
        method: NotifyMethod,
        body: String,
    ) -> Result<NotificationTemplateResponse, ServiceError> {
        let template_type_str = template_type_to_string(template_type);
        let method_str = notify_method_to_string(method);

        // Check if template already exists
        let exists = self
            .notification_repo
            .check_template_exists(branch_id, &template_type_str, &method_str)
            .await
            .map_err(ServiceError::DatabaseError)?;

        if exists {
            return Err(ServiceError::ConflictError(
                "Notification template already exists".to_string(),
            ));
        }

        let template = self
            .notification_repo
            .create_notification_template(branch_id, &template_type_str, &method_str, &body)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(template_to_response(template))
    }

    async fn update_notification_template(
        &self,
        template_id: Uuid,
        template_type: Option<NotificationTemplateType>,
        method: Option<NotifyMethod>,
        body: Option<String>,
    ) -> Result<NotificationTemplateResponse, ServiceError> {
        let template_type_str = template_type.map(template_type_to_string);
        let method_str = method.map(notify_method_to_string);

        let template = self
            .notification_repo
            .update_notification_template(template_id, template_type_str, method_str, body)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    ServiceError::NotFound("Notification template not found".to_string())
                }
                _ => ServiceError::DatabaseError(e),
            })?;

        Ok(template_to_response(template))
    }

    async fn delete_notification_template(&self, template_id: Uuid) -> Result<(), ServiceError> {
        self.notification_repo
            .delete_notification_template(template_id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(())
    }

    async fn send_bulk_notification(
        &self,
        branch_id: Uuid,
        recipients: BulkRecipientStrategy,
        method: NotifyMethod,
        message: String,
        scheduled_at: Option<DateTime<Utc>>,
    ) -> Result<BulkNotificationResponse, ServiceError> {
        let send_time = scheduled_at.unwrap_or_else(Utc::now);
        let bulk_id = Uuid::new_v4();

        let (strategy, from_date, to_date, customer_ids) = match recipients {
            BulkRecipientStrategy::AllCustomers => ("AllCustomers", None, None, vec![]),
            BulkRecipientStrategy::RecentBookings { from_date, to_date } => {
                let from_dt = DateTime::parse_from_rfc3339(&from_date)
                    .map_err(|_| {
                        ServiceError::ValidationError("Invalid from_date format".to_string())
                    })?
                    .with_timezone(&Utc);
                let to_dt = DateTime::parse_from_rfc3339(&to_date)
                    .map_err(|_| {
                        ServiceError::ValidationError("Invalid to_date format".to_string())
                    })?
                    .with_timezone(&Utc);
                ("RecentBookings", Some(from_dt), Some(to_dt), vec![])
            }
            BulkRecipientStrategy::Specific { customer_ids } => {
                ("Specific", None, None, customer_ids)
            }
        };

        let customers = self
            .notification_repo
            .get_customers_for_bulk_notification(
                branch_id,
                strategy,
                from_date,
                to_date,
                &customer_ids,
            )
            .await
            .map_err(ServiceError::DatabaseError)?;

        let mut scheduled_count = 0;
        let mut failed_count = 0;
        let mut errors = Vec::new();

        for (customer_id, phone, telegram_id) in customers.iter() {
            let can_send = match method {
                NotifyMethod::Sms => phone.is_some(),
                NotifyMethod::Telegram => telegram_id.is_some(),
            };

            if can_send {
                // In a real implementation, you would schedule the notification here
                // For now, we'll simulate success
                scheduled_count += 1;

                // Simulate sending the notification immediately if scheduled_at is None
                if scheduled_at.is_none() {
                    match method {
                        NotifyMethod::Sms => {
                            if let Some(phone_num) = phone {
                                if let Err(e) = self
                                    .sms_provider
                                    .send_notification(phone_num, &message)
                                    .await
                                {
                                    errors.push(BulkNotificationError {
                                        customer_id: *customer_id,
                                        error: format!("SMS send failed: {}", e),
                                    });
                                    failed_count += 1;
                                    scheduled_count -= 1;
                                }
                            }
                        }
                        NotifyMethod::Telegram => {
                            if let Some(tg_id) = telegram_id {
                                if let Err(e) =
                                    self.telegram_provider.send_message(*tg_id, &message).await
                                {
                                    errors.push(BulkNotificationError {
                                        customer_id: *customer_id,
                                        error: format!("Telegram send failed: {}", e),
                                    });
                                    failed_count += 1;
                                    scheduled_count -= 1;
                                }
                            }
                        }
                    }
                }
            } else {
                failed_count += 1;
                let error_msg = match method {
                    NotifyMethod::Sms => "No phone number",
                    NotifyMethod::Telegram => "No telegram",
                };
                errors.push(BulkNotificationError {
                    customer_id: *customer_id,
                    error: error_msg.to_string(),
                });
            }
        }

        Ok(BulkNotificationResponse {
            bulk_id,
            total_recipients: customers.len(),
            scheduled_count,
            failed_count,
            scheduled_at: send_time.to_rfc3339(),
            errors,
        })
    }

    async fn get_scheduled_notifications(
        &self,
        branch_id: Uuid,
    ) -> Result<Vec<ScheduledNotificationResponse>, ServiceError> {
        let notifications = self
            .notification_repo
            .get_scheduled_notifications(branch_id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(notifications
            .into_iter()
            .map(scheduled_notification_to_response)
            .collect())
    }

    async fn get_booking_notifications(
        &self,
        booking_id: Uuid,
    ) -> Result<Vec<ScheduledNotificationResponse>, ServiceError> {
        let notifications = self
            .notification_repo
            .get_booking_notifications(booking_id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(notifications
            .into_iter()
            .map(scheduled_notification_to_response)
            .collect())
    }
}

// Helper functions for conversions
fn settings_to_response(settings: NotificationSettings) -> NotificationSettingsResponse {
    NotificationSettingsResponse {
        branch_id: settings.branch_id,
        quiet_hours_start: settings.quiet_hours_start.format("%H:%M:%S").to_string(),
        quiet_hours_end: settings.quiet_hours_end.format("%H:%M:%S").to_string(),
        smart_boundary_hours: settings.smart_boundary_hours,
        critical_threshold_hours: settings.critical_threshold_hours,
        created_at: settings.created_at.to_rfc3339(),
        updated_at: settings.updated_at.to_rfc3339(),
    }
}

fn template_to_response(template: NotificationTemplate) -> NotificationTemplateResponse {
    NotificationTemplateResponse {
        id: template.id,
        branch_id: template.branch_id,
        template_type: string_to_template_type(&template.template_type),
        method: string_to_notify_method(&template.method),
        body: template.body,
        created_at: template.created_at.to_rfc3339(),
        updated_at: template.updated_at.to_rfc3339(),
    }
}

fn scheduled_notification_to_response(
    notification: ScheduledNotification,
) -> ScheduledNotificationResponse {
    ScheduledNotificationResponse {
        id: notification.id,
        booking_id: notification.booking_id,
        method: string_to_notify_method(&notification.method),
        scheduled_at: notification.scheduled_at.to_rfc3339(),
        actual_send_at: notification.actual_send_at.to_rfc3339(),
        sent_at: notification.sent_at.map(|dt| dt.to_rfc3339()),
        status: string_to_notification_status(&notification.status),
        error_message: notification.error_message,
        created_at: notification.created_at.to_rfc3339(),
        updated_at: notification.updated_at.to_rfc3339(),
    }
}

fn template_type_to_string(template_type: NotificationTemplateType) -> String {
    match template_type {
        NotificationTemplateType::BookingReminder => "booking_reminder".to_string(),
        NotificationTemplateType::BookingConfirmed => "booking_confirmed".to_string(),
        NotificationTemplateType::BookingCancelled => "booking_cancelled".to_string(),
        NotificationTemplateType::BookingRescheduled => "booking_rescheduled".to_string(),
        NotificationTemplateType::CustomMessage => "custom_message".to_string(),
    }
}

fn string_to_template_type(s: &str) -> NotificationTemplateType {
    match s {
        "booking_reminder" => NotificationTemplateType::BookingReminder,
        "booking_confirmed" => NotificationTemplateType::BookingConfirmed,
        "booking_cancelled" => NotificationTemplateType::BookingCancelled,
        "booking_rescheduled" => NotificationTemplateType::BookingRescheduled,
        "custom_message" => NotificationTemplateType::CustomMessage,
        _ => NotificationTemplateType::CustomMessage,
    }
}

fn notify_method_to_string(method: NotifyMethod) -> String {
    match method {
        NotifyMethod::Sms => "sms".to_string(),
        NotifyMethod::Telegram => "telegram".to_string(),
    }
}

fn string_to_notify_method(s: &str) -> NotifyMethod {
    match s {
        "sms" => NotifyMethod::Sms,
        "telegram" => NotifyMethod::Telegram,
        _ => NotifyMethod::Sms,
    }
}

fn string_to_notification_status(s: &str) -> NotificationStatus {
    match s {
        "pending" => NotificationStatus::Pending,
        "sent" => NotificationStatus::Sent,
        "failed" => NotificationStatus::Failed,
        "cancelled" => NotificationStatus::Cancelled,
        _ => NotificationStatus::Pending,
    }
}
