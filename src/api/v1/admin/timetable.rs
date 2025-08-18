use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        error::ApiError,
        timetable::{
            request::{
                CreateDayRedefinitionRequest, CreateTimetableRequest, UpdateTimetableRequest,
            },
            response::{DayRedefinitionOut, ScheduleDayOut, TimetableOut},
        },
    },
};

use super::ListQuery;

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(create_timetable))
        .routes(routes!(list_timetables))
        .routes(routes!(get_timetable_with_redefinitions))
        .routes(routes!(update_timetable))
        .routes(routes!(delete_timetable))
        .routes(routes!(create_day_redefinition))
        .routes(routes!(delete_day_redefinition))
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TimetableWithRedefinitionsOut {
    pub timetable: TimetableOut,
    pub schedule_days: Vec<ScheduleDayOut>,
    pub redefinitions: Vec<DayRedefinitionOut>,
}

#[utoipa::path(
    post,
    path = "/timetables",
    request_body = CreateTimetableRequest,
    responses(
        (status = 204, description = "Timetable created"),
        (status = 404, description = "Master not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), Manager (branch), or Master (own schedule) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
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
    get,
    path = "/timetables",
    params(
        ListQuery
    ),
    responses(
        (status = 200, description = "List of timetables", body = Vec<TimetableOut>),
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
pub async fn list_timetables(
    Query(query): Query<ListQuery>,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<TimetableOut>>, ApiError> {
    // TODO: Implement list timetables logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    get,
    path = "/timetables/{master_id}",
    params(
        ("master_id" = Uuid, Path, description = "Master ID")
    ),
    responses(
        (status = 200, description = "Timetable with schedule days and redefinitions", body = TimetableWithRedefinitionsOut),
        (status = 404, description = "Timetable not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), Manager (branch), or Master (own schedule) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
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
    path = "/timetables/{master_id}",
    params(
        ("master_id" = Uuid, Path, description = "Master ID")
    ),
    request_body = UpdateTimetableRequest,
    responses(
        (status = 200, description = "Timetable updated", body = TimetableOut),
        (status = 404, description = "Timetable not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), Manager (branch), or Master (own schedule) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
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
    path = "/timetables/{master_id}",
    params(
        ("master_id" = Uuid, Path, description = "Master ID")
    ),
    responses(
        (status = 204, description = "Timetable deleted"),
        (status = 404, description = "Timetable not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), or Manager (branch) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
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

#[utoipa::path(
    post,
    path = "/timetables/redefinitions",
    request_body = CreateDayRedefinitionRequest,
    responses(
        (status = 204, description = "Day redefinition created"),
        (status = 404, description = "Master not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), Manager (branch), or Master (own schedule) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
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

#[utoipa::path(
    delete,
    path = "/timetables/redefinitions/{master_id}/{date}",
    params(
        ("master_id" = Uuid, Path, description = "Master ID"),
        ("date" = NaiveDate, Path, description = "Date of the redefinition")
    ),
    responses(
        (status = 204, description = "Day redefinition deleted"),
        (status = 404, description = "Day redefinition not found", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), Manager (branch), or Master (own schedule) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
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
