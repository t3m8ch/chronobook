use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        error::ApiError,
        notification::{
            BulkNotificationResponse, CreateNotificationTemplateRequest,
            DeleteNotificationTemplateRequest, NotificationSettingsResponse,
            NotificationTemplateResponse, ScheduledNotificationResponse,
            SendBulkNotificationRequest, UpdateNotificationSettingsRequest,
            UpdateNotificationTemplateRequest,
        },
        validation::ValidationExt,
    },
};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_notification_settings))
        .routes(routes!(update_notification_settings))
        .routes(routes!(get_notification_templates))
        .routes(routes!(create_notification_template))
        .routes(routes!(get_notification_template))
        .routes(routes!(update_notification_template))
        .routes(routes!(delete_notification_template))
        .routes(routes!(send_bulk_notification))
        .routes(routes!(get_scheduled_notifications))
        .routes(routes!(get_booking_notifications))
}

#[utoipa::path(
    get,
    path = "/branches/{branch_id}/notification-settings",
    security(("bearerAuth" = [])),
    params(
        ("branch_id" = Uuid, Path, description = "Branch ID")
    ),
    responses(
        (status = 200, description = "Notification settings retrieved", body = NotificationSettingsResponse),
        (status = 404, description = "Branch not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root, Owner (organization), or Manager (branch) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn get_notification_settings(
    State(_state): State<AppState>,
    Path(_branch_id): Path<Uuid>,
) -> Result<Json<NotificationSettingsResponse>, ApiError> {
    // TODO: Implement notification settings retrieval
    todo!("Implement get_notification_settings")
}

#[utoipa::path(
    put,
    path = "/notification-settings",
    security(("bearerAuth" = [])),
    request_body = UpdateNotificationSettingsRequest,
    responses(
        (status = 200, description = "Notification settings updated", body = NotificationSettingsResponse),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 404, description = "Branch not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root, Owner (organization), or Manager (branch) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn update_notification_settings(
    State(_state): State<AppState>,
    Json(request): Json<UpdateNotificationSettingsRequest>,
) -> Result<Json<NotificationSettingsResponse>, ApiError> {
    request.validate_ext()?;
    // TODO: Implement notification settings update
    todo!("Implement update_notification_settings")
}

#[utoipa::path(
    get,
    path = "/branches/{branch_id}/notification-templates",
    security(("bearerAuth" = [])),
    params(
        ("branch_id" = Uuid, Path, description = "Branch ID")
    ),
    responses(
        (status = 200, description = "Notification templates retrieved", body = Vec<NotificationTemplateResponse>),
        (status = 404, description = "Branch not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root or Owner (organization) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn get_notification_templates(
    State(_state): State<AppState>,
    Path(_branch_id): Path<Uuid>,
) -> Result<Json<Vec<NotificationTemplateResponse>>, ApiError> {
    // TODO: Implement notification templates retrieval
    todo!("Implement get_notification_templates")
}

#[utoipa::path(
    post,
    path = "/notification-templates",
    security(("bearerAuth" = [])),
    request_body = CreateNotificationTemplateRequest,
    responses(
        (status = 201, description = "Notification template created", body = NotificationTemplateResponse),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 404, description = "Branch not found", body = ApiError),
        (status = 409, description = "Template already exists", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root or Owner (organization) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn create_notification_template(
    State(_state): State<AppState>,
    Json(request): Json<CreateNotificationTemplateRequest>,
) -> Result<(StatusCode, Json<NotificationTemplateResponse>), ApiError> {
    request.validate_ext()?;
    // TODO: Implement notification template creation
    todo!("Implement create_notification_template")
}

#[utoipa::path(
    get,
    path = "/branches/{branch_id}/notification-templates/{template_id}",
    security(("bearerAuth" = [])),
    params(
        ("branch_id" = Uuid, Path, description = "Branch ID"),
        ("template_id" = Uuid, Path, description = "Template ID")
    ),
    responses(
        (status = 200, description = "Notification template retrieved", body = NotificationTemplateResponse),
        (status = 404, description = "Template not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root or Owner (organization) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn get_notification_template(
    State(_state): State<AppState>,
    Path((_branch_id, _template_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<NotificationTemplateResponse>, ApiError> {
    // TODO: Implement notification template retrieval
    todo!("Implement get_notification_template")
}

#[utoipa::path(
    put,
    path = "/notification-templates",
    security(("bearerAuth" = [])),
    request_body = UpdateNotificationTemplateRequest,
    responses(
        (status = 200, description = "Notification template updated", body = NotificationTemplateResponse),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 404, description = "Template not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root or Owner (organization) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn update_notification_template(
    State(_state): State<AppState>,
    Json(request): Json<UpdateNotificationTemplateRequest>,
) -> Result<Json<NotificationTemplateResponse>, ApiError> {
    request.validate_ext()?;
    // TODO: Implement notification template update
    todo!("Implement update_notification_template")
}

#[utoipa::path(
    delete,
    path = "/notification-templates",
    security(("bearerAuth" = [])),
    request_body = DeleteNotificationTemplateRequest,
    responses(
        (status = 204, description = "Notification template deleted"),
        (status = 404, description = "Template not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root or Owner (organization) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn delete_notification_template(
    State(_state): State<AppState>,
    Json(request): Json<DeleteNotificationTemplateRequest>,
) -> Result<StatusCode, ApiError> {
    request.validate_ext()?;
    // TODO: Implement notification template deletion
    todo!("Implement delete_notification_template")
}

#[utoipa::path(
    post,
    path = "/send-bulk-notification",
    security(("bearerAuth" = [])),
    request_body = SendBulkNotificationRequest,
    responses(
        (status = 200, description = "Bulk notification sent", body = BulkNotificationResponse),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 404, description = "Branch not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root, Owner (organization), or Manager (branch) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn send_bulk_notification(
    State(_state): State<AppState>,
    Json(request): Json<SendBulkNotificationRequest>,
) -> Result<Json<BulkNotificationResponse>, ApiError> {
    request.validate_ext()?;
    // TODO: Implement bulk notification sending
    todo!("Implement send_bulk_notification")
}

#[utoipa::path(
    get,
    path = "/branches/{branch_id}/scheduled-notifications",
    security(("bearerAuth" = [])),
    params(
        ("branch_id" = Uuid, Path, description = "Branch ID")
    ),
    responses(
        (status = 200, description = "Scheduled notifications retrieved", body = Vec<ScheduledNotificationResponse>),
        (status = 404, description = "Branch not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root, Owner (organization), or Manager (branch) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn get_scheduled_notifications(
    State(_state): State<AppState>,
    Path(_branch_id): Path<Uuid>,
) -> Result<Json<Vec<ScheduledNotificationResponse>>, ApiError> {
    // TODO: Implement scheduled notifications retrieval
    todo!("Implement get_scheduled_notifications")
}

#[utoipa::path(
    get,
    path = "/bookings/{booking_id}/notifications",
    security(("bearerAuth" = [])),
    params(
        ("booking_id" = Uuid, Path, description = "Booking ID")
    ),
    responses(
        (status = 200, description = "Booking notifications retrieved", body = Vec<ScheduledNotificationResponse>),
        (status = 404, description = "Booking not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root, Owner (organization), or Manager (branch) role", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
pub async fn get_booking_notifications(
    State(_state): State<AppState>,
    Path(_booking_id): Path<Uuid>,
) -> Result<Json<Vec<ScheduledNotificationResponse>>, ApiError> {
    // TODO: Implement booking notifications retrieval
    todo!("Implement get_booking_notifications")
}
