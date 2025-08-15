use poem_openapi::{ApiResponse, Object, payload::Json};
use uuid::Uuid;

use crate::models::error::ApiError;

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetServicesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ServiceOut>>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct ServiceOut {
    id: Uuid,
    name: String,
    description: String,
    duration_minutes: Option<u32>,
    price: String,
}
