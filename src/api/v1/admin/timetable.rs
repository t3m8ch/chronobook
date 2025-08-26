use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use axum_valid::Garde;
use chrono::NaiveDate;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        error::{ApiError, ErrorType},
        timetable::{
            request::{
                CreateDayRedefinitionRequest, CreateTimetableRequest, UpdateTimetableRequest,
            },
            response::{TimetableOut, TimetableWithRedefinitionsOut},
        },
    },
};

use super::ListQuery;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_timetable))
        .routes(routes!(list_timetables))
        .routes(routes!(get_timetable_with_redefinitions))
        .routes(routes!(update_timetable))
        .routes(routes!(delete_timetable))
        .routes(routes!(create_day_redefinition))
        .routes(routes!(delete_day_redefinition))
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
    tag = "admin/timetable"
)]
#[tracing::instrument(skip(state))]
pub async fn create_timetable(
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<CreateTimetableRequest>>,
) -> Result<StatusCode, ApiError> {
    state
        .timetable_service
        .create_timetable(request)
        .await
        .map_err(|e| ApiError::new(ErrorType::InternalServer, &e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
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
    tag = "admin/timetable"
)]
#[tracing::instrument(skip(state))]
pub async fn list_timetables(
    Query(query): Query<ListQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<TimetableOut>>, ApiError> {
    let timetables = state
        .timetable_service
        .list_timetables(query.organization_id)
        .await
        .map_err(|e| ApiError::new(ErrorType::InternalServer, &e.to_string()))?;
    Ok(Json(timetables))
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
    tag = "admin/timetable"
)]
#[tracing::instrument(skip(state))]
pub async fn get_timetable_with_redefinitions(
    Path(master_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<TimetableWithRedefinitionsOut>, ApiError> {
    let timetable_with_redefinitions = state
        .timetable_service
        .get_timetable_with_redefinitions(master_id)
        .await
        .map_err(|e| match &e {
            crate::services::errors::ServiceError::NotFound(_) => {
                ApiError::new(ErrorType::NotFound, "Timetable not found")
            }
            _ => ApiError::new(ErrorType::InternalServer, &e.to_string()),
        })?;
    Ok(Json(timetable_with_redefinitions))
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
    tag = "admin/timetable"
)]
#[tracing::instrument(skip(state))]
pub async fn update_timetable(
    Path(master_id): Path<Uuid>,
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<UpdateTimetableRequest>>,
) -> Result<Json<TimetableOut>, ApiError> {
    let updated_timetable = state
        .timetable_service
        .update_timetable(master_id, request)
        .await
        .map_err(|e| match &e {
            crate::services::errors::ServiceError::NotFound(_) => {
                ApiError::new(ErrorType::NotFound, "Timetable not found")
            }
            _ => ApiError::new(ErrorType::InternalServer, &e.to_string()),
        })?;
    Ok(Json(updated_timetable))
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
    tag = "admin/timetable"
)]
#[tracing::instrument(skip(state))]
pub async fn delete_timetable(
    Path(master_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    state
        .timetable_service
        .delete_timetable(master_id)
        .await
        .map_err(|e| match &e {
            crate::services::errors::ServiceError::NotFound(_) => {
                ApiError::new(ErrorType::NotFound, "Timetable not found")
            }
            _ => ApiError::new(ErrorType::InternalServer, &e.to_string()),
        })?;
    Ok(StatusCode::NO_CONTENT)
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
    tag = "admin/timetable"
)]
#[tracing::instrument(skip(state))]
pub async fn create_day_redefinition(
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<CreateDayRedefinitionRequest>>,
) -> Result<StatusCode, ApiError> {
    state
        .timetable_service
        .create_day_redefinition(request)
        .await
        .map_err(|e| match &e {
            crate::services::errors::ServiceError::NotFound(_) => {
                ApiError::new(ErrorType::NotFound, "Timetable not found for this master")
            }
            _ => ApiError::new(ErrorType::InternalServer, &e.to_string()),
        })?;
    Ok(StatusCode::NO_CONTENT)
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
    tag = "admin/timetable"
)]
#[tracing::instrument(skip(state))]
pub async fn delete_day_redefinition(
    Path((master_id, date)): Path<(Uuid, NaiveDate)>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    state
        .timetable_service
        .delete_day_redefinition(master_id, date)
        .await
        .map_err(|e| match &e {
            crate::services::errors::ServiceError::NotFound(_) => {
                ApiError::new(ErrorType::NotFound, "Day redefinition not found")
            }
            _ => ApiError::new(ErrorType::InternalServer, &e.to_string()),
        })?;
    Ok(StatusCode::NO_CONTENT)
}
