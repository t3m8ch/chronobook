use poem_openapi::{ApiResponse, Object, payload::Json};
use uuid::Uuid;

use crate::models::error::ApiError;

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum CreateEmployeeResponse {
    #[oai(status = 201)]
    Created(Json<CreateEmployeeOut>),

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
pub struct CreateEmployeeOut {
    pub id: Uuid,
}
