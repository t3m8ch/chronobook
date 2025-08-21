use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum_valid::Garde;
use std::sync::Arc;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::AppState;
use crate::extractors::AuthUser;
use crate::models::error::ApiError;
use crate::models::organization::request::CreateOrganizationRequest;
use crate::models::organization::response::OrganizationOut;
use crate::services::organization::OrganizationService;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(create_organization))
}

#[utoipa::path(
    post,
    path = "/organizations",
    tag = "admin",
    request_body = CreateOrganizationRequest,
    responses(
        (status = 201, description = "Organization created successfully", body = OrganizationOut),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 409, description = "Organization name already exists", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub async fn create_organization(
    State(organization_service): State<Arc<dyn OrganizationService>>,
    auth_user: AuthUser,
    Garde(Json(request)): Garde<Json<CreateOrganizationRequest>>,
) -> Result<(StatusCode, Json<OrganizationOut>), ApiError> {
    let organization = organization_service
        .create_organization(auth_user.user_id, request)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(OrganizationOut {
            id: organization.id.to_string(),
            name: organization.name,
            display_name: organization.display_name,
            description: organization.description.unwrap_or_default(),
        }),
    ))
}
