use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use axum_valid::Garde;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppState,
    extractors::auth::AuthUser,
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

pub fn router() -> OpenApiRouter<AppState> {
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
        ("organization_name" = String, Path, description = "Organization name"),
    ),
    responses(
        (status = 200, description = "Organization with name", body = OrganizationOut),
        (status = 404, description = "Organization not found", body = OrganizationOut),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument(skip(state))]
pub async fn get_organization_by_name(
    State(state): State<AppState>,
    Path(organization_name): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let organization = state
        .booking_service
        .get_organization_by_name(&organization_name)
        .await?;
    Ok(Json(organization))
}

#[utoipa::path(
    get,
    path = "/services",
    params(
        ("organization_name" = String, Query, description = "Organization name"),
        ("masters[]" = Vec<Uuid>, Query, description = "Master IDs filter")
    ),
    responses(
        (status = 200, description = "List of services", body = Vec<ServiceOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument(skip(state))]
pub async fn get_services(
    State(state): State<AppState>,
    Garde(Query(query)): Garde<Query<GetServicesQuery>>,
) -> Result<impl IntoResponse, ApiError> {
    let services = state
        .booking_service
        .get_services(&query.organization_name, &query.masters)
        .await?;
    Ok(Json(services))
}

#[utoipa::path(
    get,
    path = "/masters",
    params(
        ("organization_name" = String, Query, description = "Organization name"),
        ("branches[]" = Vec<Uuid>, Query, description = "Branch IDs filter"),
        ("services[]" = Vec<Uuid>, Query, description = "Service IDs filter"),
    ),
    responses(
        (status = 200, description = "List of masters", body = Vec<MasterOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument(skip(state))]
pub async fn get_masters(
    State(state): State<AppState>,
    Garde(Query(query)): Garde<Query<GetMastersQuery>>,
) -> Result<impl IntoResponse, ApiError> {
    let masters = state
        .booking_service
        .get_masters(&query.organization_name, &query.branches, &query.services)
        .await?;
    Ok(Json(masters))
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
#[tracing::instrument(skip(state))]
pub async fn get_master_by_id(
    State(state): State<AppState>,
    Path(master_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let master = state.booking_service.get_master_by_id(master_id).await?;
    Ok(Json(master))
}

#[utoipa::path(
    get,
    path = "/branches",
    params(
        ("organization_name" = String, Query, description = "Organization name"),
        ("masters[]" = Vec<Uuid>, Query, description = "Master IDs filter")
    ),
    responses(
        (status = 200, description = "List of branches", body = Vec<BranchOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
#[tracing::instrument(skip(state))]
pub async fn get_branches(
    State(state): State<AppState>,
    Garde(Query(query)): Garde<Query<GetBranchesQuery>>,
) -> Result<impl IntoResponse, ApiError> {
    let branches = state
        .booking_service
        .get_branches(&query.organization_name, &query.masters)
        .await?;
    Ok(Json(branches))
}

#[utoipa::path(
    get,
    path = "/windows",
    params(
        ("organization_name" = String, Query, description = "Organization name"),
        ("service_id" = Uuid, Query, description = "Service ID"),
        ("masters[]" = Vec<Uuid>, Query, description = "Master IDs filter"),
        ("branches[]" = Vec<Uuid>, Query, description = "Branch IDs filter"),
        ("min_datetime" = NaiveDateTime, Query, description = "Min datetime"),
        ("max_datetime" = NaiveDateTime, Query, description = "Max datetime"),
    ),
    responses(
        (status = 200, description = "List of branches", body = Vec<BranchOut>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "bookings"
)]
async fn get_windows(
    State(state): State<AppState>,
    Garde(Query(query)): Garde<Query<GetWindowsQuery>>,
) -> Result<impl IntoResponse, ApiError> {
    let windows = state.booking_service.get_windows(&query).await?;
    Ok(Json(windows))
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
    State(state): State<AppState>,
    auth_user: AuthUser,
    Garde(Json(request)): Garde<Json<CreateBookingRequest>>,
) -> Result<impl IntoResponse, ApiError> {
    let booking = state
        .booking_service
        .create_booking(auth_user.user_id, &request)
        .await?;
    Ok(Json(booking))
}
