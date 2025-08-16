use axum::{
    Json,
    extract::{Query, State},
    response::IntoResponse,
    routing::{Router, get},
};
use std::sync::Arc;

use crate::{
    AppState,
    models::{
        branch::{request::GetBranchesQuery, response::BranchOut},
        error::ApiError,
        master::response::{GetMastersQuery, MasterOut},
        service::{request::GetServicesQuery, response::ServiceOut},
    },
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/services", get(get_services))
        .route("/masters", get(get_masters))
        .route("/branches", get(get_branches))
}

#[utoipa::path(
    get,
    path = "/api/v1/bookings/services",
    params(
        ("organization_id" = Uuid, Query, description = "Organization ID")
    ),
    responses(
        (status = 200, description = "List of services", body = Vec<ServiceOut>),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument]
pub async fn get_services(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<GetServicesQuery>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement get services logic
    Ok(Json(Vec::<ServiceOut>::new()))
}

#[utoipa::path(
    get,
    path = "/api/v1/bookings/masters",
    params(
        ("organization_id" = Uuid, Query, description = "Organization ID"),
        ("branches[]" = Vec<Uuid>, Query, description = "Branch IDs filter")
    ),
    responses(
        (status = 200, description = "List of masters", body = Vec<MasterOut>),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument]
pub async fn get_masters(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<GetMastersQuery>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement get masters logic
    Ok(Json(Vec::<MasterOut>::new()))
}

#[utoipa::path(
    get,
    path = "/api/v1/bookings/branches",
    params(
        ("organization_id" = Uuid, Query, description = "Organization ID"),
        ("masters[]" = Vec<Uuid>, Query, description = "Master IDs filter")
    ),
    responses(
        (status = 200, description = "List of branches", body = Vec<BranchOut>),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument]
pub async fn get_branches(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<GetBranchesQuery>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement get branches logic
    Ok(Json(Vec::<BranchOut>::new()))
}
