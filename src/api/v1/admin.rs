use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        branch::{request::CreateBranchRequest, response::CreateBranchOut},
        dashboard::response::OrganizationDashboardOut,
        employee::{request::CreateEmployeeRequest, response::CreateEmployeeOut},
        error::ApiError,
        service::{request::CreateServiceRequest, response::CreateServiceOut},
    },
};

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(get_organization_dashboard))
        .routes(routes!(create_branch))
        .routes(routes!(create_employee))
        .routes(routes!(create_service))
}

#[utoipa::path(
    get,
    path = "/dashboard/{organization_id}",
    params(
        ("organization_id" = Uuid, Path, description = "Organization ID")
    ),
    responses(
        (status = 200, description = "Organization dashboard", body = OrganizationDashboardOut),
        (status = 404, description = "Organization not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Invalid token", body = ApiError),
        (status = 400, description = "Token expired", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn get_organization_dashboard(
    Path(organization_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<OrganizationDashboardOut>, ApiError> {
    // TODO: Implement get organization dashboard logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    post,
    path = "/branches",
    request_body = CreateBranchRequest,
    responses(
        (status = 201, description = "Branch created", body = CreateBranchOut),
        (status = 404, description = "Organization not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Invalid token", body = ApiError),
        (status = 400, description = "Token expired", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn create_branch(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<CreateBranchRequest>,
) -> Result<Json<CreateBranchOut>, ApiError> {
    // TODO: Implement create branch logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    post,
    path = "/employees",
    request_body = CreateEmployeeRequest,
    responses(
        (status = 201, description = "Employee created", body = CreateEmployeeOut),
        (status = 404, description = "Organization not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Invalid token", body = ApiError),
        (status = 400, description = "Token expired", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn create_employee(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<CreateEmployeeRequest>,
) -> Result<Json<CreateEmployeeOut>, ApiError> {
    // TODO: Implement create employee logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    post,
    path = "/services",
    request_body = CreateServiceRequest,
    responses(
        (status = 201, description = "Service created", body = CreateServiceOut),
        (status = 404, description = "Organization not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Invalid token", body = ApiError),
        (status = 400, description = "Token expired", body = ApiError),
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
