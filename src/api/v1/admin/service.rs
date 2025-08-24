use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use axum_valid::Garde;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    extractors::auth::AuthUser,
    models::{
        error::ApiError,
        service::{
            request::{CreateServiceRequest, UpdateServiceRequest},
            response::{CreateServiceOut, ServiceOut},
        },
    },
    services::errors::ServiceError,
};

use super::ListQuery;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_service))
        .routes(routes!(list_services))
        .routes(routes!(get_service))
        .routes(routes!(update_service))
        .routes(routes!(delete_service))
}

#[utoipa::path(
    post,
    path = "/services",
    security(("bearerAuth" = [])),
    request_body = CreateServiceRequest,
    responses(
        (status = 201, description = "Service created", body = CreateServiceOut),
        (status = 404, description = "Organization not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root or Owner (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument(skip(state))]
pub async fn create_service(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<CreateServiceRequest>>,
) -> Result<Json<CreateServiceOut>, ApiError> {
    let organization_id = auth_user.get_organization_id().ok_or_else(|| {
        ApiError::new(
            "MISSING_ORGANIZATION",
            "User must belong to an organization",
        )
    })?;

    let result = state
        .service_service
        .create_service(request, organization_id)
        .await
        .map_err(|e| match e {
            ServiceError::ValidationError(msg) => ApiError::new("VALIDATION_ERROR", &msg),
            ServiceError::DatabaseError(_) => ApiError::new("INTERNAL_ERROR", "Database error"),
            _ => ApiError::new("INTERNAL_ERROR", "Internal server error"),
        })?;

    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/services",
    security(("bearerAuth" = [])),
    params(
        ListQuery
    ),
    responses(
        (status = 200, description = "List of services", body = Vec<ServiceOut>),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root, Owner (organization), Manager (organization), or Master (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument(skip(state))]
pub async fn list_services(
    auth_user: AuthUser,
    Query(query): Query<ListQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ServiceOut>>, ApiError> {
    let organization_id = auth_user.get_organization_id().ok_or_else(|| {
        ApiError::new(
            "MISSING_ORGANIZATION",
            "User must belong to an organization",
        )
    })?;

    let limit = query.limit.unwrap_or(50).min(100) as i64;
    let offset = query.offset.unwrap_or(0) as i64;

    let services = state
        .service_service
        .list_services(organization_id, limit, offset)
        .await
        .map_err(|e| match e {
            ServiceError::DatabaseError(_) => ApiError::new("INTERNAL_ERROR", "Database error"),
            _ => ApiError::new("INTERNAL_ERROR", "Internal server error"),
        })?;

    Ok(Json(services))
}

#[utoipa::path(
    get,
    path = "/services/{service_id}",
    security(("bearerAuth" = [])),
    params(
        ("service_id" = Uuid, Path, description = "Service ID")
    ),
    responses(
        (status = 200, description = "Service details", body = ServiceOut),
        (status = 404, description = "Service not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root, Owner (organization), Manager (organization), or Master (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument(skip(state))]
pub async fn get_service(
    auth_user: AuthUser,
    Path(service_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<ServiceOut>, ApiError> {
    let organization_id = auth_user.get_organization_id().ok_or_else(|| {
        ApiError::new(
            "MISSING_ORGANIZATION",
            "User must belong to an organization",
        )
    })?;

    let service = state
        .service_service
        .get_service(service_id, organization_id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound => ApiError::new("NOT_FOUND", "Service not found"),
            ServiceError::DatabaseError(_) => ApiError::new("INTERNAL_ERROR", "Database error"),
            _ => ApiError::new("INTERNAL_ERROR", "Internal server error"),
        })?;

    Ok(Json(service))
}

#[utoipa::path(
    put,
    path = "/services/{service_id}",
    security(("bearerAuth" = [])),
    params(
        ("service_id" = Uuid, Path, description = "Service ID")
    ),
    request_body = UpdateServiceRequest,
    responses(
        (status = 200, description = "Service updated", body = ServiceOut),
        (status = 404, description = "Service not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root or Owner (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument(skip(state))]
pub async fn update_service(
    auth_user: AuthUser,
    Path(service_id): Path<Uuid>,
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<UpdateServiceRequest>>,
) -> Result<Json<ServiceOut>, ApiError> {
    let organization_id = auth_user.get_organization_id().ok_or_else(|| {
        ApiError::new(
            "MISSING_ORGANIZATION",
            "User must belong to an organization",
        )
    })?;

    let service = state
        .service_service
        .update_service(service_id, request, organization_id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound => ApiError::new("NOT_FOUND", "Service not found"),
            ServiceError::ValidationError(msg) => ApiError::new("VALIDATION_ERROR", &msg),
            ServiceError::DatabaseError(_) => ApiError::new("INTERNAL_ERROR", "Database error"),
            _ => ApiError::new("INTERNAL_ERROR", "Internal server error"),
        })?;

    Ok(Json(service))
}

#[utoipa::path(
    delete,
    path = "/services/{service_id}",
    security(("bearerAuth" = [])),
    params(
        ("service_id" = Uuid, Path, description = "Service ID")
    ),
    responses(
        (status = 204, description = "Service deleted"),
        (status = 404, description = "Service not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - requires Root or Owner (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument(skip(state))]
pub async fn delete_service(
    auth_user: AuthUser,
    Path(service_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    let organization_id = auth_user.get_organization_id().ok_or_else(|| {
        ApiError::new(
            "MISSING_ORGANIZATION",
            "User must belong to an organization",
        )
    })?;

    state
        .service_service
        .delete_service(service_id, organization_id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound => ApiError::new("NOT_FOUND", "Service not found"),
            ServiceError::DatabaseError(_) => ApiError::new("INTERNAL_ERROR", "Database error"),
            _ => ApiError::new("INTERNAL_ERROR", "Internal server error"),
        })?;

    Ok(StatusCode::NO_CONTENT)
}
