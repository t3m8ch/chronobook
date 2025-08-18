use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        error::ApiError,
        service::{
            request::{CreateServiceRequest, UpdateServiceRequest},
            response::{CreateServiceOut, ServiceOut},
        },
    },
};

use super::ListQuery;

pub fn router() -> OpenApiRouter<Arc<AppState>> {
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
#[tracing::instrument]
pub async fn create_service(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<CreateServiceRequest>,
) -> Result<Json<CreateServiceOut>, ApiError> {
    // TODO: Implement create service logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
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
#[tracing::instrument]
pub async fn list_services(
    Query(query): Query<ListQuery>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<ServiceOut>>, ApiError> {
    // TODO: Implement list services logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
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
#[tracing::instrument]
pub async fn get_service(
    Path(service_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<ServiceOut>, ApiError> {
    // TODO: Implement get service logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
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
#[tracing::instrument]
pub async fn update_service(
    Path(service_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
    Json(request): Json<UpdateServiceRequest>,
) -> Result<Json<ServiceOut>, ApiError> {
    // TODO: Implement update service logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
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
#[tracing::instrument]
pub async fn delete_service(
    Path(service_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<StatusCode, ApiError> {
    // TODO: Implement delete service logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}
