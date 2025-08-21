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
        branch::{
            request::{CreateBranchRequest, UpdateBranchRequest},
            response::{BranchOut, CreateBranchOut},
        },
        error::ApiError,
    },
};

use super::ListQuery;

pub fn router() -> OpenApiRouter<AppState> {
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
#[tracing::instrument(skip(state))]
pub async fn create_branch(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<CreateBranchRequest>>,
) -> Result<Json<CreateBranchOut>, ApiError> {
    let result = state
        .branch_service
        .create_branch(auth_user.user_id, request)
        .await?;

    Ok(Json(result))
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
#[tracing::instrument(skip(state))]
pub async fn list_branches(
    auth_user: AuthUser,
    Garde(Query(query)): Garde<Query<ListQuery>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BranchOut>>, ApiError> {
    let limit = query.limit.unwrap_or(20).min(100) as i64;
    let offset = query.offset.unwrap_or(0) as i64;

    let branches = state
        .branch_service
        .list_branches(auth_user.user_id, query.organization_id, limit, offset)
        .await?;

    Ok(Json(branches))
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
#[tracing::instrument(skip(state))]
pub async fn get_branch(
    auth_user: AuthUser,
    Path(branch_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<BranchOut>, ApiError> {
    let branch = state
        .branch_service
        .get_branch(auth_user.user_id, branch_id)
        .await?;

    Ok(Json(branch))
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
#[tracing::instrument(skip(state))]
pub async fn update_branch(
    auth_user: AuthUser,
    Path(branch_id): Path<Uuid>,
    State(state): State<AppState>,
    Garde(Json(request)): Garde<Json<UpdateBranchRequest>>,
) -> Result<Json<BranchOut>, ApiError> {
    let branch = state
        .branch_service
        .update_branch(auth_user.user_id, branch_id, request)
        .await?;

    Ok(Json(branch))
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
#[tracing::instrument(skip(state))]
pub async fn delete_branch(
    auth_user: AuthUser,
    Path(branch_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    state
        .branch_service
        .delete_branch(auth_user.user_id, branch_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
