use async_trait::async_trait;
use chrono::{DateTime, NaiveTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::notification::db::{
    NotificationSettings, NotificationTemplate, ScheduledNotification,
};

#[mockall::automock]
#[async_trait]
pub trait NotificationRepository: Send + Sync {
    // Notification Settings
    async fn get_notification_settings(
        &self,
        branch_id: Uuid,
    ) -> Result<Option<NotificationSettings>, sqlx::Error>;
    async fn create_default_notification_settings(
        &self,
        branch_id: Uuid,
    ) -> Result<NotificationSettings, sqlx::Error>;
    async fn update_notification_settings(
        &self,
        branch_id: Uuid,
        quiet_hours_start: Option<NaiveTime>,
        quiet_hours_end: Option<NaiveTime>,
        smart_boundary_hours: Option<i32>,
        critical_threshold_hours: Option<i32>,
    ) -> Result<NotificationSettings, sqlx::Error>;

    // Notification Templates
    async fn get_notification_templates(
        &self,
        branch_id: Uuid,
    ) -> Result<Vec<NotificationTemplate>, sqlx::Error>;
    async fn get_notification_template(
        &self,
        branch_id: Uuid,
        template_id: Uuid,
    ) -> Result<Option<NotificationTemplate>, sqlx::Error>;
    async fn create_notification_template(
        &self,
        branch_id: Uuid,
        template_type: &str,
        method: &str,
        body: &str,
    ) -> Result<NotificationTemplate, sqlx::Error>;
    async fn update_notification_template(
        &self,
        template_id: Uuid,
        template_type: Option<String>,
        method: Option<String>,
        body: Option<String>,
    ) -> Result<NotificationTemplate, sqlx::Error>;
    async fn delete_notification_template(&self, template_id: Uuid) -> Result<(), sqlx::Error>;
    async fn check_template_exists(
        &self,
        branch_id: Uuid,
        template_type: &str,
        method: &str,
    ) -> Result<bool, sqlx::Error>;

    // Scheduled Notifications
    async fn get_scheduled_notifications(
        &self,
        branch_id: Uuid,
    ) -> Result<Vec<ScheduledNotification>, sqlx::Error>;
    async fn get_booking_notifications(
        &self,
        booking_id: Uuid,
    ) -> Result<Vec<ScheduledNotification>, sqlx::Error>;
    async fn create_scheduled_notification(
        &self,
        booking_id: Uuid,
        method: &str,
        template_id: Uuid,
        scheduled_at: DateTime<Utc>,
        actual_send_at: DateTime<Utc>,
    ) -> Result<ScheduledNotification, sqlx::Error>;
    async fn get_customers_for_bulk_notification(
        &self,
        branch_id: Uuid,
        strategy: &str,
        from_date: Option<DateTime<Utc>>,
        to_date: Option<DateTime<Utc>>,
        customer_ids: &[Uuid],
    ) -> Result<Vec<(Uuid, Option<String>, Option<i64>)>, sqlx::Error>;
}

pub struct PostgresNotificationRepository {
    pool: PgPool,
}

impl PostgresNotificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    async fn get_notification_settings(
        &self,
        branch_id: Uuid,
    ) -> Result<Option<NotificationSettings>, sqlx::Error> {
        let settings = sqlx::query_as::<_, NotificationSettings>(
            "SELECT branch_id, created_at, updated_at, quiet_hours_start, quiet_hours_end, smart_boundary_hours, critical_threshold_hours FROM notification_settings WHERE branch_id = $1"
        )
        .bind(branch_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(settings)
    }

    async fn create_default_notification_settings(
        &self,
        branch_id: Uuid,
    ) -> Result<NotificationSettings, sqlx::Error> {
        let now = Utc::now();
        let settings = sqlx::query_as::<_, NotificationSettings>(
            "INSERT INTO notification_settings (branch_id, created_at, updated_at, quiet_hours_start, quiet_hours_end, smart_boundary_hours, critical_threshold_hours) 
            VALUES ($1, $2, $3, '22:00:00', '08:00:00', 12, 3) 
            RETURNING branch_id, created_at, updated_at, quiet_hours_start, quiet_hours_end, smart_boundary_hours, critical_threshold_hours"
        )
        .bind(branch_id)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(settings)
    }

    async fn update_notification_settings(
        &self,
        branch_id: Uuid,
        quiet_hours_start: Option<NaiveTime>,
        quiet_hours_end: Option<NaiveTime>,
        smart_boundary_hours: Option<i32>,
        critical_threshold_hours: Option<i32>,
    ) -> Result<NotificationSettings, sqlx::Error> {
        let now = Utc::now();
        let settings = sqlx::query_as::<_, NotificationSettings>(
            "UPDATE notification_settings 
            SET updated_at = $2,
                quiet_hours_start = COALESCE($3, quiet_hours_start),
                quiet_hours_end = COALESCE($4, quiet_hours_end),
                smart_boundary_hours = COALESCE($5, smart_boundary_hours),
                critical_threshold_hours = COALESCE($6, critical_threshold_hours)
            WHERE branch_id = $1
            RETURNING branch_id, created_at, updated_at, quiet_hours_start, quiet_hours_end, smart_boundary_hours, critical_threshold_hours"
        )
        .bind(branch_id)
        .bind(now)
        .bind(quiet_hours_start)
        .bind(quiet_hours_end)
        .bind(smart_boundary_hours)
        .bind(critical_threshold_hours)
        .fetch_one(&self.pool)
        .await?;

        Ok(settings)
    }

    async fn get_notification_templates(
        &self,
        branch_id: Uuid,
    ) -> Result<Vec<NotificationTemplate>, sqlx::Error> {
        let templates = sqlx::query_as::<_, NotificationTemplate>(
            "SELECT id, created_at, updated_at, branch_id, template_type, method, body FROM notification_templates WHERE branch_id = $1 ORDER BY template_type, method"
        )
        .bind(branch_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(templates)
    }

    async fn get_notification_template(
        &self,
        branch_id: Uuid,
        template_id: Uuid,
    ) -> Result<Option<NotificationTemplate>, sqlx::Error> {
        let template = sqlx::query_as::<_, NotificationTemplate>(
            "SELECT id, created_at, updated_at, branch_id, template_type, method, body FROM notification_templates WHERE branch_id = $1 AND id = $2"
        )
        .bind(branch_id)
        .bind(template_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(template)
    }

    async fn create_notification_template(
        &self,
        branch_id: Uuid,
        template_type: &str,
        method: &str,
        body: &str,
    ) -> Result<NotificationTemplate, sqlx::Error> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let template = sqlx::query_as::<_, NotificationTemplate>(
            "INSERT INTO notification_templates (id, created_at, updated_at, branch_id, template_type, method, body) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, created_at, updated_at, branch_id, template_type, method, body"
        )
        .bind(id)
        .bind(now)
        .bind(now)
        .bind(branch_id)
        .bind(template_type)
        .bind(method)
        .bind(body)
        .fetch_one(&self.pool)
        .await?;

        Ok(template)
    }

    async fn update_notification_template(
        &self,
        template_id: Uuid,
        template_type: Option<String>,
        method: Option<String>,
        body: Option<String>,
    ) -> Result<NotificationTemplate, sqlx::Error> {
        let now = Utc::now();

        let template = sqlx::query_as::<_, NotificationTemplate>(
            "UPDATE notification_templates 
            SET updated_at = $2,
                template_type = COALESCE($3, template_type),
                method = COALESCE($4, method),
                body = COALESCE($5, body)
            WHERE id = $1
            RETURNING id, created_at, updated_at, branch_id, template_type, method, body",
        )
        .bind(template_id)
        .bind(now)
        .bind(template_type)
        .bind(method)
        .bind(body)
        .fetch_one(&self.pool)
        .await?;

        Ok(template)
    }

    async fn delete_notification_template(&self, template_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM notification_templates WHERE id = $1")
            .bind(template_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn check_template_exists(
        &self,
        branch_id: Uuid,
        template_type: &str,
        method: &str,
    ) -> Result<bool, sqlx::Error> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM notification_templates WHERE branch_id = $1 AND template_type = $2 AND method = $3"
        )
        .bind(branch_id)
        .bind(template_type)
        .bind(method)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    async fn get_scheduled_notifications(
        &self,
        branch_id: Uuid,
    ) -> Result<Vec<ScheduledNotification>, sqlx::Error> {
        let notifications = sqlx::query_as::<_, ScheduledNotification>(
            "SELECT sn.id, sn.created_at, sn.updated_at, sn.booking_id, sn.method, sn.template_id, 
            sn.scheduled_at, sn.actual_send_at, sn.sent_at, sn.status, sn.error_message
            FROM scheduled_notifications sn
            JOIN bookings b ON sn.booking_id = b.id
            WHERE b.branch_id = $1
            ORDER BY sn.scheduled_at DESC",
        )
        .bind(branch_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(notifications)
    }

    async fn get_booking_notifications(
        &self,
        booking_id: Uuid,
    ) -> Result<Vec<ScheduledNotification>, sqlx::Error> {
        let notifications = sqlx::query_as::<_, ScheduledNotification>(
            "SELECT id, created_at, updated_at, booking_id, method, template_id, 
            scheduled_at, actual_send_at, sent_at, status, error_message
            FROM scheduled_notifications 
            WHERE booking_id = $1
            ORDER BY scheduled_at DESC",
        )
        .bind(booking_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(notifications)
    }

    async fn create_scheduled_notification(
        &self,
        booking_id: Uuid,
        method: &str,
        template_id: Uuid,
        scheduled_at: DateTime<Utc>,
        actual_send_at: DateTime<Utc>,
    ) -> Result<ScheduledNotification, sqlx::Error> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let notification = sqlx::query_as::<_, ScheduledNotification>(
            "INSERT INTO scheduled_notifications (id, created_at, updated_at, booking_id, method, template_id, scheduled_at, actual_send_at, status) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'pending')
            RETURNING id, created_at, updated_at, booking_id, method, template_id, scheduled_at, actual_send_at, sent_at, status, error_message"
        )
        .bind(id)
        .bind(now)
        .bind(now)
        .bind(booking_id)
        .bind(method)
        .bind(template_id)
        .bind(scheduled_at)
        .bind(actual_send_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(notification)
    }

    async fn get_customers_for_bulk_notification(
        &self,
        branch_id: Uuid,
        strategy: &str,
        from_date: Option<DateTime<Utc>>,
        to_date: Option<DateTime<Utc>>,
        customer_ids: &[Uuid],
    ) -> Result<Vec<(Uuid, Option<String>, Option<i64>)>, sqlx::Error> {
        let customers = match strategy {
            "AllCustomers" => {
                sqlx::query_as::<_, (Uuid, Option<String>, Option<i64>)>(
                    "SELECT c.id, u.phone, u.telegram_id 
                    FROM customers c
                    JOIN users u ON c.user_id = u.id
                    JOIN organizations o ON c.organization_id = o.id
                    JOIN branches b ON o.id = b.organization_id
                    WHERE b.id = $1",
                )
                .bind(branch_id)
                .fetch_all(&self.pool)
                .await?
            }
            "RecentBookings" => {
                sqlx::query_as::<_, (Uuid, Option<String>, Option<i64>)>(
                    "SELECT DISTINCT c.id, u.phone, u.telegram_id 
                    FROM customers c
                    JOIN users u ON c.user_id = u.id
                    JOIN bookings bk ON c.id = bk.customer_id
                    WHERE bk.branch_id = $1 
                    AND bk.started_at >= $2 
                    AND bk.started_at <= $3",
                )
                .bind(branch_id)
                .bind(from_date.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30)))
                .bind(to_date.unwrap_or_else(Utc::now))
                .fetch_all(&self.pool)
                .await?
            }
            "Specific" => {
                if customer_ids.is_empty() {
                    return Ok(vec![]);
                }
                let query = format!(
                    "SELECT c.id, u.phone, u.telegram_id 
                    FROM customers c
                    JOIN users u ON c.user_id = u.id
                    WHERE c.id = ANY($1::uuid[])"
                );
                sqlx::query_as::<_, (Uuid, Option<String>, Option<i64>)>(&query)
                    .bind(customer_ids)
                    .fetch_all(&self.pool)
                    .await?
            }
            _ => return Ok(vec![]),
        };

        Ok(customers)
    }
}
