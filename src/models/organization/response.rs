use poem_openapi::{ApiResponse, Object, payload::Json};

use crate::models::error::ApiError;

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetOrganizationByNameResponse {
    #[oai(status = 200)]
    Ok(Json<OrganizationOut>),

    #[oai(status = 404)]
    NotFound(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct OrganizationOut {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
}
