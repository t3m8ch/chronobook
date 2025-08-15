use poem_openapi::{ApiResponse, Object, payload::Json};
use uuid::Uuid;

use crate::models::error::ApiError;

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetBookingMastersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<MasterOut>>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetAdminMastersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<MasterOut>>),

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

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetMasterByIdResponse {
    #[oai(status = 200)]
    Ok(Json<MasterOut>),

    #[oai(status = 404)]
    NotFound(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetMasterByNameResponse {
    #[oai(status = 200)]
    Ok(Json<MasterOut>),

    #[oai(status = 404)]
    NotFound(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct MasterOut {
    id: Uuid,
    first_name: String,
    last_name: String,
    patronymic: Option<String>,
    contact_phone: Option<String>,
    contact_email: Option<String>,
    contact_telegram: Option<String>,
}
