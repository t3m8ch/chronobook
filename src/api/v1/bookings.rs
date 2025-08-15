use chrono::{DateTime, Duration, Utc};
use poem_openapi::{OpenApi, param::Query, payload::Json};
use uuid::Uuid;

use crate::models::{
    booking::{
        request::CreateBookingRequest,
        response::{CreateBookingResponse, GetWindowsResponse},
    },
    branch::response::GetBranchesResponse,
    master::response::GetMastersResponse,
    service::response::GetServicesResponse,
};

#[derive(Clone, Debug)]
pub struct BookingsApi;

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

    #[tracing::instrument]
    #[oai(path = "/", method = "post")]
    async fn create_booking(
        &self,
        Json(request): Json<CreateBookingRequest>,
    ) -> CreateBookingResponse {
        todo!()
    }
}

fn default_min_datetime() -> DateTime<Utc> {
    Utc::now()
}

fn default_max_datetime() -> DateTime<Utc> {
    Utc::now() + Duration::days(30)
}
