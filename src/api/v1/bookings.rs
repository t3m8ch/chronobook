use chrono::{DateTime, Duration, Utc};
use poem_openapi::{ApiResponse, Object, OpenApi, param::Query, payload::Json};
use uuid::Uuid;

use crate::api::error::ApiError;

#[derive(Clone, Debug)]
pub struct BookingsApi;

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetServicesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ServiceOut>>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetMastersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<MasterOut>>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetBranchesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<BranchOut>>),

    #[oai(status = 500)]
    InternalServerError(Json<ApiError>),
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetWindowsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<WindowOut>>),

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

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct WindowOut {
    id: Uuid,
    slots: Vec<SlotOut>,
    master: MasterOut,
    branch: BranchOut,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct SlotOut {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

#[OpenApi(prefix_path = "/bookings")]
impl BookingsApi {
    #[tracing::instrument]
    #[oai(path = "/services", method = "get")]
    async fn get_services(&self, Query(organization_id): Query<Uuid>) -> GetServicesResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/masters", method = "get")]
    async fn get_masters(
        &self,
        Query(organization_id): Query<Uuid>,
        Query(branches): Query<Vec<Uuid>>,
    ) -> GetMastersResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/branches", method = "get")]
    async fn get_branches(
        &self,
        Query(organization_id): Query<Uuid>,
        Query(masters): Query<Vec<Uuid>>,
    ) -> GetBranchesResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/windows", method = "get")]
    async fn get_windows(
        &self,
        Query(organization_id): Query<Uuid>,
        Query(masters): Query<Vec<Uuid>>,
        Query(branches): Query<Vec<Uuid>>,
        #[oai(default = "default_min_datetime")] Query(min_datetime): Query<DateTime<Utc>>,
        #[oai(default = "default_max_datetime")] Query(max_datetime): Query<DateTime<Utc>>,
    ) -> GetWindowsResponse {
        todo!()
    }
}

fn default_min_datetime() -> DateTime<Utc> {
    Utc::now()
}

fn default_max_datetime() -> DateTime<Utc> {
    Utc::now() + Duration::days(30)
}
