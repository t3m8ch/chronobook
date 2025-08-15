use poem_openapi::{ApiResponse, Object, payload::Json};
use uuid::Uuid;

use crate::models::error::ApiError;

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetMastersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<MasterOut>>),

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
