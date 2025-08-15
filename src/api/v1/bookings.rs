use chrono::{DateTime, Duration, Utc};
use poem_openapi::{
    OpenApi,
    param::{Path, Query},
    payload::Json,
};
use uuid::Uuid;

use crate::models::{
    booking::{
        request::CreateBookingRequest,
        response::{CreateBookingResponse, GetWindowsResponse},
    },
    branch::response::GetBookingBranchesResponse,
    master::response::{GetBookingMastersResponse, GetMasterByIdResponse},
    organization::response::GetOrganizationByNameResponse,
    service::response::GetServicesResponse,
};

#[derive(Clone, Debug)]
pub struct BookingsApi;

#[OpenApi(prefix_path = "/bookings")]
impl BookingsApi {
    #[tracing::instrument]
    #[oai(path = "/organizations/{organization_name}", method = "get")]
    async fn get_organization_by_name(
        &self,
        Path(organization_name): Path<String>,
    ) -> GetOrganizationByNameResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/services", method = "get")]
    async fn get_services(&self, Query(organization_name): Query<String>) -> GetServicesResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/masters", method = "get")]
    async fn get_masters(
        &self,
        Query(organization_name): Query<String>,
        Query(branches): Query<Vec<Uuid>>,
    ) -> GetBookingMastersResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/masters/{master_id}", method = "get")]
    async fn get_master_by_id(&self, Path(master_id): Path<Uuid>) -> GetMasterByIdResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/branches", method = "get")]
    async fn get_branches(
        &self,
        Query(organization_name): Query<String>,
        Query(masters): Query<Vec<Uuid>>,
    ) -> GetBookingBranchesResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/windows", method = "get")]
    async fn get_windows(
        &self,
        Query(organization_name): Query<String>,
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
