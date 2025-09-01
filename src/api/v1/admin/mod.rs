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
pub mod organizations;
pub mod service;
pub mod timetable;

use crate::{
    AppState,
    extractors::auth::AuthUser,
    models::{
        dashboard::response::OrganizationDashboardOut,
        error::{ApiError, ErrorType},
    },
    services::jwt::UserRole,
};

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_organization_dashboard))
        .merge(branch::router())
        .merge(employee::router())
        .merge(notification::router())
        .merge(organizations::router())
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
#[tracing::instrument(skip(state, auth_user))]
pub async fn get_organization_dashboard(
    Path(organization_id): Path<Uuid>,
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<OrganizationDashboardOut>, ApiError> {
    // Check if user has access to this organization
    // User must be either an employee with Owner/Manager/Master role or a customer of this organization
    let required_roles = vec![UserRole::Owner, UserRole::Manager, UserRole::Master];

    if !auth_user.has_access_to_organization(organization_id)
        && !auth_user.has_role_in_organization(organization_id, &required_roles)
    {
        return Err(ApiError::forbidden("Access denied to this organization"));
    }

    let dashboard = state
        .dashboard_service
        .get_organization_dashboard(organization_id)
        .await
        .map_err(|e| match e {
            crate::services::errors::ServiceError::NotFound(_) => {
                ApiError::new(ErrorType::NotFound, "Organization not found")
            }
            _ => ApiError::new(ErrorType::InternalServer, &e.to_string()),
        })?;

    Ok(Json(dashboard))
}
