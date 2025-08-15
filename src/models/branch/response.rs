use poem_openapi::{ApiResponse, Object, payload::Json};
use uuid::Uuid;

use crate::models::error::ApiError;

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetBranchesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<BranchOut>>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct BranchOut {
    id: Uuid,
    name: String,
    description: String,
    timezone: String,
    street: String,
    house_number: String,
    apartment_number: String,
    city: String,
    region: String,
    country: String,
    address_info: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum CreateBranchResponse {
    #[oai(status = 201)]
    Created(Json<CreateBranchOut>),

    #[oai(status = 404)]
    NotFound(Json<ApiError>),

    #[oai(status = 401)]
    Unauthorized(Json<ApiError>),

    #[oai(status = 403)]
    Forbidden(Json<ApiError>),

    #[oai(status = 400)]
    InvalidToken(Json<ApiError>),

    #[oai(status = 400)]
    TokenExpired(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct CreateBranchOut {
    id: Uuid,
}
