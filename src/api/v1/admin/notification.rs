use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_valid::Garde;
use chrono::{DateTime, Utc};
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
    },
    services::errors::ServiceError,
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
    tag = "admin/notification"
)]
pub async fn get_notification_settings(
    State(state): State<AppState>,
    Path(branch_id): Path<Uuid>,
) -> Result<Json<NotificationSettingsResponse>, ApiError> {
    let settings = state
        .notification_service
        .get_notification_settings(branch_id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok(Json(settings))
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
    tag = "admin/notification"
)]
pub async fn update_notification_settings(
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<UpdateNotificationSettingsRequest>>,
) -> Result<Json<NotificationSettingsResponse>, ApiError> {
    let settings = state
        .notification_service
        .update_notification_settings(
            request.branch_id,
            request.quiet_hours_start,
            request.quiet_hours_end,
            request.smart_boundary_hours,
            request.critical_threshold_hours,
        )
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::ValidationError(msg) => ApiError::bad_request(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok(Json(settings))
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
    tag = "admin/notification"
)]
pub async fn get_notification_templates(
    State(state): State<AppState>,
    Path(branch_id): Path<Uuid>,
) -> Result<Json<Vec<NotificationTemplateResponse>>, ApiError> {
    let templates = state
        .notification_service
        .get_notification_templates(branch_id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok(Json(templates))
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
    tag = "admin/notification"
)]
pub async fn create_notification_template(
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<CreateNotificationTemplateRequest>>,
) -> Result<(StatusCode, Json<NotificationTemplateResponse>), ApiError> {
    let template = state
        .notification_service
        .create_notification_template(
            request.branch_id,
            request.template_type,
            request.method,
            request.body,
        )
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::ConflictError(msg) => ApiError::conflict(msg),
            ServiceError::ValidationError(msg) => ApiError::bad_request(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok((StatusCode::CREATED, Json(template)))
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
    tag = "admin/notification"
)]
pub async fn get_notification_template(
    State(state): State<AppState>,
    Path((branch_id, template_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<NotificationTemplateResponse>, ApiError> {
    let template = state
        .notification_service
        .get_notification_template(branch_id, template_id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok(Json(template))
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
    tag = "admin/notification"
)]
pub async fn update_notification_template(
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<UpdateNotificationTemplateRequest>>,
) -> Result<Json<NotificationTemplateResponse>, ApiError> {
    let template = state
        .notification_service
        .update_notification_template(
            request.template_id,
            request.template_type,
            request.method,
            request.body,
        )
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::ValidationError(msg) => ApiError::bad_request(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok(Json(template))
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
    tag = "admin/notification"
)]
pub async fn delete_notification_template(
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<DeleteNotificationTemplateRequest>>,
) -> Result<StatusCode, ApiError> {
    state
        .notification_service
        .delete_notification_template(request.template_id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok(StatusCode::NO_CONTENT)
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
    tag = "admin/notification"
)]
pub async fn send_bulk_notification(
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<SendBulkNotificationRequest>>,
) -> Result<Json<BulkNotificationResponse>, ApiError> {
    let scheduled_at = if let Some(scheduled_str) = request.scheduled_at {
        Some(
            DateTime::parse_from_rfc3339(&scheduled_str)
                .map_err(|_| ApiError::bad_request("Invalid scheduled_at format"))?
                .with_timezone(&Utc),
        )
    } else {
        None
    };

    let response = state
        .notification_service
        .send_bulk_notification(
            request.branch_id,
            request.recipients,
            request.method,
            request.message,
            scheduled_at,
        )
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::ValidationError(msg) => ApiError::bad_request(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok(Json(response))
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
    tag = "admin/notification"
)]
pub async fn get_scheduled_notifications(
    State(state): State<AppState>,
    Path(branch_id): Path<Uuid>,
) -> Result<Json<Vec<ScheduledNotificationResponse>>, ApiError> {
    let notifications = state
        .notification_service
        .get_scheduled_notifications(branch_id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok(Json(notifications))
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
    tag = "admin/notification"
)]
pub async fn get_booking_notifications(
    State(state): State<AppState>,
    Path(booking_id): Path<Uuid>,
) -> Result<Json<Vec<ScheduledNotificationResponse>>, ApiError> {
    let notifications = state
        .notification_service
        .get_booking_notifications(booking_id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(msg) => ApiError::not_found(msg),
            ServiceError::DatabaseError(_) => ApiError::internal_server_error(e.to_string()),
            _ => ApiError::internal_server_error(e.to_string()),
        })?;

    Ok(Json(notifications))
}
