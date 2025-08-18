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
        branch::{
            request::{CreateBranchRequest, UpdateBranchRequest},
            response::{BranchOut, CreateBranchOut},
        },
        error::ApiError,
    },
};

use super::ListQuery;

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(create_branch))
        .routes(routes!(list_branches))
        .routes(routes!(get_branch))
        .routes(routes!(update_branch))
        .routes(routes!(delete_branch))
}

#[utoipa::path(
    post,
    path = "/branches",
    request_body = CreateBranchRequest,
    responses(
        (status = 201, description = "Branch created", body = CreateBranchOut),
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
pub async fn create_branch(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<CreateBranchRequest>,
) -> Result<Json<CreateBranchOut>, ApiError> {
    // TODO: Implement create branch logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}

#[utoipa::path(
    get,
    path = "/branches",
    params(
        ListQuery
    ),
    responses(
        (status = 200, description = "List of branches", body = Vec<BranchOut>),
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
        (status = 403, description = "Forbidden - Requires Root, Owner (organization), Manager (branch), or Master (organization) role", body = ApiError),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
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
pub async fn delete_branch(
    Path(branch_id): Path<Uuid>,
    State(_state): State<Arc<AppState>>,
) -> Result<StatusCode, ApiError> {
    // TODO: Implement delete branch logic
    Err(ApiError::new("NOT_IMPLEMENTED", "Not implemented"))
}
