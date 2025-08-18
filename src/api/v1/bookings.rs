use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    models::{
        booking::{request::CreateBookingRequest, response::BookingOut},
        branch::{request::GetBranchesQuery, response::BranchOut},
        error::ApiError,
        master::response::{GetMastersQuery, MasterOut},
        organization::response::OrganizationOut,
        service::{request::GetServicesQuery, response::ServiceOut},
        timetable::{request::GetWindowsQuery, response::WindowOut},
    },
};

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(get_organization_by_name))
        .routes(routes!(get_services))
        .routes(routes!(get_masters))
        .routes(routes!(get_master_by_id))
        .routes(routes!(get_branches))
        .routes(routes!(get_windows))
        .routes(routes!(create_booking))
}

#[utoipa::path(
    get,
    path = "/organizations/{organization_name}",
    params(
        ("organization_name" = String, Path, description = "Organization name")
    ),
    responses(
        (status = 200, description = "Organization with name", body = OrganizationOut),
        (status = 404, description = "Organization not found", body = OrganizationOut),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument(skip(_state))]
pub async fn get_organization_by_name(
    State(_state): State<Arc<AppState>>,
    Path(organization_name): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement
    Ok(Json(OrganizationOut::default()))
}

#[utoipa::path(
    get,
    path = "/services",
    params(
        ("organization_name" = String, Query, description = "Organization name")
    ),
    responses(
        (status = 200, description = "List of services", body = Vec<ServiceOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument(skip(_state))]
pub async fn get_services(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<GetServicesQuery>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement get services logic
    Ok(Json(Vec::<ServiceOut>::new()))
}

#[utoipa::path(
    get,
    path = "/masters",
    params(
        ("organization_name" = String, Query, description = "Organization name"),
        ("branches[]" = Vec<Uuid>, Query, description = "Branch IDs filter")
    ),
    responses(
        (status = 200, description = "List of masters", body = Vec<MasterOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument(skip(_state))]
pub async fn get_masters(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<GetMastersQuery>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement get masters logic
    Ok(Json(Vec::<MasterOut>::new()))
}

#[utoipa::path(
    get,
    path = "/masters/{master_id}",
    params(
        ("master_id" = String, Path, description = "Master id")
    ),
    responses(
        (status = 200, description = "Master with ID", body = MasterOut),
        (status = 404, description = "Master not found", body = MasterOut),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument(skip(_state))]
pub async fn get_master_by_id(
    State(_state): State<Arc<AppState>>,
    Path(master_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement
    Ok(Json(MasterOut::default()))
}

#[utoipa::path(
    get,
    path = "/branches",
    params(
        ("organization_name" = Uuid, Query, description = "Organization name"),
        ("masters[]" = Vec<Uuid>, Query, description = "Master IDs filter")
    ),
    responses(
        (status = 200, description = "List of branches", body = Vec<BranchOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument(skip(_state))]
pub async fn get_branches(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<GetBranchesQuery>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Implement get branches logic
    Ok(Json(Vec::<BranchOut>::new()))
}

#[utoipa::path(
    get,
    path = "/windows",
    params(
        ("organization_name" = Uuid, Query, description = "Organization name"),
        ("masters[]" = Vec<Uuid>, Query, description = "Master IDs filter"),
        ("branches[]" = Vec<Uuid>, Query, description = "Branch IDs filter"),
        ("min_datetime" = DateTime<Utc>, Query, description = "Min datetime"),
        ("max_datetime" = DateTime<Utc>, Query, description = "Max datetime"),
    ),
    responses(
        (status = 200, description = "List of branches", body = Vec<BranchOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
async fn get_windows(
    State(_state): State<Arc<AppState>>,
    Query(_query): Query<GetWindowsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(Vec::<WindowOut>::new()))
}

#[utoipa::path(
    post,
    path = "/",
    request_body = CreateBookingRequest,
    responses(
        (status = 201, description = "Booking created", body = Vec<BookingOut>),
        (status = 401, description = "Unauthorized - Authentication required", body = ApiError),
        (status = 404, description = "Service, branch or master not found", body = Vec<ApiError>),
        (status = 409, description = "Already booked", body = Vec<ApiError>),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "bookings"
)]
async fn create_booking(
    State(_state): State<Arc<AppState>>,
    Json(_request): Json<CreateBookingRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(BookingOut::default()))
}
