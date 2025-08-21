use axum::{
    Json,
    extract::{Path, State},
};
use garde::Validate;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

pub mod branch;
pub mod employee;
pub mod notification;
pub mod service;
pub mod timetable;

use crate::{
    AppState,
    models::{dashboard::response::OrganizationDashboardOut, error::ApiError},
};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_organization_dashboard))
        .merge(branch::router())
        .merge(employee::router())
        .merge(notification::router())
        .merge(service::router())
        .merge(timetable::router())
}

#[derive(Debug, Clone, Deserialize, IntoParams, Validate)]
pub struct ListQuery {
    #[garde(skip)]
    /// Organization ID (optional for filtering)
    pub organization_id: Option<Uuid>,

    #[garde(skip)]
    /// Limit number of results (default: 20, max: 100)
    pub limit: Option<usize>,

    #[garde(skip)]
    /// Offset for pagination
    pub offset: Option<usize>,
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
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), Manager (organization), or Master (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "admin"
)]
#[tracing::instrument(skip(_state))]
pub async fn get_organization_dashboard(
    Path(organization_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> Result<Json<OrganizationDashboardOut>, ApiError> {
    // TODO: Implement get organization dashboard logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}
