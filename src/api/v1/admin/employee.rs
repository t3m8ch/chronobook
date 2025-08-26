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
        employee::{
            request::{CreateEmployeeRequest, UpdateEmployeeRequest},
            response::{CreateEmployeeOut, EmployeeOut},
        },
        error::{ApiError, ErrorType},
    },
};

use super::ListQuery;

pub fn router() -> OpenApiRouter<AppState> {
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
    tag = "admin/employee"
)]
#[tracing::instrument(skip(state))]
pub async fn create_employee(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(request)): Garde<Json<CreateEmployeeRequest>>,
) -> Result<Json<CreateEmployeeOut>, ApiError> {
    // Ensure user has an organization context
    let org_id = auth_user
        .organization_id
        .ok_or_else(|| ApiError::new(ErrorType::NotFound, "Organization context required"))?;

    let result = state
        .employee_service
        .create_employee(auth_user.user_id, org_id, request)
        .await?;

    Ok(Json(result))
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
    tag = "admin/employee"
)]
#[tracing::instrument(skip(state))]
pub async fn list_employees(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<EmployeeOut>>, ApiError> {
    // Ensure user has an organization context
    let org_id = auth_user
        .organization_id
        .ok_or_else(|| ApiError::new(ErrorType::NotFound, "Organization context required"))?;

    let limit = query.limit.unwrap_or(20) as i64;
    let offset = query.offset.unwrap_or(0) as i64;

    let employees = state
        .employee_service
        .list_employees(auth_user.user_id, org_id, limit, offset)
        .await?;

    Ok(Json(employees))
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
    tag = "admin/employee"
)]
#[tracing::instrument(skip(state))]
pub async fn get_employee(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(employee_id): Path<Uuid>,
) -> Result<Json<EmployeeOut>, ApiError> {
    let employee = state
        .employee_service
        .get_employee(auth_user.user_id, employee_id)
        .await?;

    Ok(Json(employee))
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
    tag = "admin/employee"
)]
#[tracing::instrument(skip(state))]
pub async fn update_employee(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(employee_id): Path<Uuid>,
    Garde(Json(request)): Garde<Json<UpdateEmployeeRequest>>,
) -> Result<Json<EmployeeOut>, ApiError> {
    let employee = state
        .employee_service
        .update_employee(auth_user.user_id, employee_id, request)
        .await?;

    Ok(Json(employee))
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
    tag = "admin/employee"
)]
#[tracing::instrument(skip(state))]
pub async fn delete_employee(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(employee_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    state
        .employee_service
        .delete_employee(auth_user.user_id, employee_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
