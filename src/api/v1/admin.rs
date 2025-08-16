use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
    routing::{Router, get, post},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        branch::response::BranchOut,
        branch::{request::CreateBranchRequest, response::CreateBranchOut},
        dashboard::response::OrganizationDashboardOut,
        employee::{request::CreateEmployeeRequest, response::CreateEmployeeOut},
        error::ApiError,
        master::response::MasterOut,
        service::{request::CreateServiceRequest, response::CreateServiceOut},
    },
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/organizations", post(create_organization))
        .route(
            "/dashboard/{organization_id}",
            get(get_organization_dashboard),
        )
        .route("/branches", post(create_branch))
        .route("/branches", get(get_branches))
        .route("/masters", get(get_masters))
        .route("/employees", post(create_employee))
        .route("/services", post(create_service))
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/organizations",
    responses(
        (status = 201, description = "Organization created"),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn create_organization(
    State(_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement create organization logic
    Ok(())
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/dashboard/{organization_id}",
    params(
        ("organization_id" = Uuid, Path, description = "Organization ID")
    ),
    responses(
        (status = 200, description = "Organization dashboard", body = OrganizationDashboardOut),
        (status = 404, description = "Organization not found", body = ApiError),
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
    path = "/api/v1/admin/branches",
    request_body = CreateBranchRequest,
    responses(
        (status = 201, description = "Branch created", body = CreateBranchOut),
        (status = 400, description = "Bad request", body = ApiError),
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
    get,
    path = "/api/v1/admin/branches",
    responses(
        (status = 200, description = "List of branches", body = Vec<BranchOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn get_branches(
    State(_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement get branches logic
    Ok(Json(Vec::<BranchOut>::new()))
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/masters",
    responses(
        (status = 200, description = "List of masters", body = Vec<MasterOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn get_masters(
    State(_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement get masters logic
    Ok(Json(Vec::<MasterOut>::new()))
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/employees",
    request_body = CreateEmployeeRequest,
    responses(
        (status = 201, description = "Employee created", body = CreateEmployeeOut),
        (status = 400, description = "Bad request", body = ApiError),
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
    path = "/api/v1/admin/services",
    request_body = CreateServiceRequest,
    responses(
        (status = 201, description = "Service created", body = CreateServiceOut),
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
