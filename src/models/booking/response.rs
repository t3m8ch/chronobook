use poem_openapi::{ApiResponse, Object, payload::Json};
use uuid::Uuid;

use crate::models::{booking::request::WindowOut, error::ApiError};

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum CreateBookingResponse {
    #[oai(status = 201)]
    Created(Json<BookingOut>),

    #[oai(status = 404)]
    NotFound(Json<ApiError>),

    #[oai(status = 409)]
    AlreadyBooked(Json<ApiError>),

    #[oai(status = 400)]
    InvalidToken(Json<ApiError>),

    #[oai(status = 400)]
    TokenExpired(Json<ApiError>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct BookingOut {
    id: Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetWindowsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<WindowOut>>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}
