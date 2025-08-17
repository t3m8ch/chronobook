use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;
use chrono::NaiveDate;

use crate::{
    AppState,
    models::{
        branch::{
            request::{CreateBranchRequest, UpdateBranchRequest},
            response::{BranchOut, CreateBranchOut},
        },
        dashboard::response::OrganizationDashboardOut,
        employee::{
            request::{CreateEmployeeRequest, UpdateEmployeeRequest},
            response::{CreateEmployeeOut, EmployeeOut},
        },
        error::ApiError,
        service::{
            request::{CreateServiceRequest, UpdateServiceRequest},
            response::{CreateServiceOut, ServiceOut},
        },
        timetable::{
            request::{CreateDayRedefinitionRequest, CreateTimetableRequest, UpdateTimetableRequest},
            response::{TimetableOut, DayRedefinitionOut, ScheduleDayOut},
        },
    },
};

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(get_organization_dashboard))
        // Branch CRUD
        .routes(routes!(create_branch))
        .routes(routes!(list_branches))
        .routes(routes!(get_branch))
        .routes(routes!(update_branch))
        .routes(routes!(delete_branch))
        // Employee CRUD
        .routes(routes!(create_employee))
        .routes(routes!(list_employees))
        .routes(routes!(get_employee))
        .routes(routes!(update_employee))
        .routes(routes!(delete_employee))
        // Service CRUD
        .routes(routes!(create_service))
        .routes(routes!(list_services))
        .routes(routes!(get_service))
        .routes(routes!(update_service))
        .routes(routes!(delete_service))
        // Timetable CRUD
        .routes(routes!(create_timetable))
        .routes(routes!(list_timetables))
        .routes(routes!(get_timetable_with_redefinitions))
        .routes(routes!(update_timetable))
        .routes(routes!(delete_timetable))
        // Day Redefinition (create and delete only)
        .routes(routes!(create_day_redefinition))
        .routes(routes!(delete_day_redefinition))
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
        (status = 400, description = "Bad request", body = ApiError),
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
    post,
    path = "/employees",
    request_body = CreateEmployeeRequest,
    responses(
        (status = 201, description = "Employee created", body = CreateEmployeeOut),
        (status = 404, description = "Organization not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
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
    path = "/services",
    request_body = CreateServiceRequest,
    responses(
        (status = 201, description = "Service created", body = CreateServiceOut),
        (status = 404, description = "Organization not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
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
    post,
    path = "/timetable",
    request_body = CreateTimetableRequest,
    responses(
        (status = 204, description = "Timetable created"),
        (status = 404, description = "Master not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn create_timetable(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<CreateTimetableRequest>,
) -> Result<StatusCode, ApiError> {
    // TODO: Implement create timetable logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    post,
    path = "/timetable/redefinitions",
    request_body = CreateDayRedefinitionRequest,
    responses(
        (status = 204, description = "Day redefinition created"),
        (status = 404, description = "Master not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn create_day_redefinition(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<CreateDayRedefinitionRequest>,
) -> Result<StatusCode, ApiError> {
    // TODO: Implement create day redefinition logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

// Query parameters for list endpoints

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct ListQuery {
    /// Organization ID
    pub organization_id: Uuid,
}

// Branch CRUD operations

#[utoipa::path(
    get,
    path = "/branches",
    params(
        ListQuery
    ),
    responses(
        (status = 200, description = "List of branches", body = Vec<BranchOut>),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn list_branches(
    Query(query): Query<ListQuery>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<BranchOut>>, ApiError> {
    // TODO: Implement list branches logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    get,
    path = "/branches/{branch_id}",
    params(
        ("branch_id" = Uuid, Path, description = "Branch ID")
    ),
    responses(
        (status = 200, description = "Branch details", body = BranchOut),
        (status = 404, description = "Branch not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn get_branch(
    Path(branch_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<BranchOut>, ApiError> {
    // TODO: Implement get branch logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    put,
    path = "/branches/{branch_id}",
    params(
        ("branch_id" = Uuid, Path, description = "Branch ID")
    ),
    request_body = UpdateBranchRequest,
    responses(
        (status = 200, description = "Branch updated", body = BranchOut),
        (status = 404, description = "Branch not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn update_branch(
    Path(branch_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
    Json(request): Json<UpdateBranchRequest>,
) -> Result<Json<BranchOut>, ApiError> {
    // TODO: Implement update branch logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    delete,
    path = "/branches/{branch_id}",
    params(
        ("branch_id" = Uuid, Path, description = "Branch ID")
    ),
    responses(
        (status = 204, description = "Branch deleted"),
        (status = 404, description = "Branch not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn delete_branch(
    Path(branch_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<StatusCode, ApiError> {
    // TODO: Implement delete branch logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

// Employee CRUD operations

#[utoipa::path(
    get,
    path = "/employees",
    params(
        ListQuery
    ),
    responses(
        (status = 200, description = "List of employees", body = Vec<EmployeeOut>),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
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
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
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
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
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
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
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

// Service CRUD operations

#[utoipa::path(
    get,
    path = "/services",
    params(
        ListQuery
    ),
    responses(
        (status = 200, description = "List of services", body = Vec<ServiceOut>),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
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
    params(
        ("service_id" = Uuid, Path, description = "Service ID")
    ),
    responses(
        (status = 200, description = "Service details", body = ServiceOut),
        (status = 404, description = "Service not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
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
    params(
        ("service_id" = Uuid, Path, description = "Service ID")
    ),
    request_body = UpdateServiceRequest,
    responses(
        (status = 200, description = "Service updated", body = ServiceOut),
        (status = 404, description = "Service not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
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
    params(
        ("service_id" = Uuid, Path, description = "Service ID")
    ),
    responses(
        (status = 204, description = "Service deleted"),
        (status = 404, description = "Service not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
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

// Timetable CRUD operations

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TimetableWithRedefinitionsOut {
    pub timetable: TimetableOut,
    pub schedule_days: Vec<ScheduleDayOut>,
    pub redefinitions: Vec<DayRedefinitionOut>,
}

#[utoipa::path(
    get,
    path = "/timetables",
    params(
        ListQuery
    ),
    responses(
        (status = 200, description = "List of timetables", body = Vec<TimetableOut>),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn list_timetables(
    Query(query): Query<ListQuery>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<TimetableOut>>, ApiError> {
    // TODO: Implement list timetables logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    get,
    path = "/timetable/{master_id}",
    params(
        ("master_id" = Uuid, Path, description = "Master ID")
    ),
    responses(
        (status = 200, description = "Timetable with schedule days and redefinitions", body = TimetableWithRedefinitionsOut),
        (status = 404, description = "Timetable not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn get_timetable_with_redefinitions(
    Path(master_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<TimetableWithRedefinitionsOut>, ApiError> {
    // TODO: Implement get timetable with redefinitions logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    put,
    path = "/timetable/{master_id}",
    params(
        ("master_id" = Uuid, Path, description = "Master ID")
    ),
    request_body = UpdateTimetableRequest,
    responses(
        (status = 200, description = "Timetable updated", body = TimetableOut),
        (status = 404, description = "Timetable not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn update_timetable(
    Path(master_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
    Json(request): Json<UpdateTimetableRequest>,
) -> Result<Json<TimetableOut>, ApiError> {
    // TODO: Implement update timetable logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    delete,
    path = "/timetable/{master_id}",
    params(
        ("master_id" = Uuid, Path, description = "Master ID")
    ),
    responses(
        (status = 204, description = "Timetable deleted"),
        (status = 404, description = "Timetable not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn delete_timetable(
    Path(master_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<StatusCode, ApiError> {
    // TODO: Implement delete timetable logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

// Day Redefinition operations

#[utoipa::path(
    delete,
    path = "/timetable/redefinitions/{master_id}/{date}",
    params(
        ("master_id" = Uuid, Path, description = "Master ID"),
        ("date" = NaiveDate, Path, description = "Date of the redefinition")
    ),
    responses(
        (status = 204, description = "Day redefinition deleted"),
        (status = 404, description = "Day redefinition not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "admin"
)]
#[tracing::instrument]
pub async fn delete_day_redefinition(
    Path((master_id, date)): Path<(Uuid, NaiveDate)>,
    State(_state): State<Arc<AppState>>,
) -> Result<StatusCode, ApiError> {
    // TODO: Implement delete day redefinition logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}
