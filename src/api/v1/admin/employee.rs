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
        employee::{
            request::{CreateEmployeeRequest, UpdateEmployeeRequest},
            response::{CreateEmployeeOut, EmployeeOut},
        },
        error::ApiError,
    },
};

use super::ListQuery;

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(create_employee))
        .routes(routes!(list_employees))
        .routes(routes!(get_employee))
        .routes(routes!(update_employee))
        .routes(routes!(delete_employee))
}

#[utoipa::path(
    post,
    path = "/employees",
    request_body = CreateEmployeeRequest,
    responses(
        (status = 201, description = "Employee created", body = CreateEmployeeOut),
        (status = 404, description = "Organization not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root or Owner (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
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
    get,
    path = "/employees",
    params(
        ListQuery
    ),
    responses(
        (status = 200, description = "List of employees", body = Vec<EmployeeOut>),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), Manager (organization), or Master (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn list_employees(
    Query(query): Query<ListQuery>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<EmployeeOut>>, ApiError> {
    // TODO: Implement list employees logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    get,
    path = "/employees/{employee_id}",
    params(
        ("employee_id" = Uuid, Path, description = "Employee ID")
    ),
    responses(
        (status = 200, description = "Employee details", body = EmployeeOut),
        (status = 404, description = "Employee not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), Manager (organization), or Master (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn get_employee(
    Path(employee_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<EmployeeOut>, ApiError> {
    // TODO: Implement get employee logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    put,
    path = "/employees/{employee_id}",
    params(
        ("employee_id" = Uuid, Path, description = "Employee ID")
    ),
    request_body = UpdateEmployeeRequest,
    responses(
        (status = 200, description = "Employee updated", body = EmployeeOut),
        (status = 404, description = "Employee not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root or Owner (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn update_employee(
    Path(employee_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
    Json(request): Json<UpdateEmployeeRequest>,
) -> Result<Json<EmployeeOut>, ApiError> {
    // TODO: Implement update employee logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    delete,
    path = "/employees/{employee_id}",
    params(
        ("employee_id" = Uuid, Path, description = "Employee ID")
    ),
    responses(
        (status = 204, description = "Employee deleted"),
        (status = 404, description = "Employee not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root or Owner (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn delete_employee(
    Path(employee_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<StatusCode, ApiError> {
    // TODO: Implement delete employee logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}
