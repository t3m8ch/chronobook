use poem_openapi::{ApiResponse, Object, OpenApi, param::Query, payload::Json};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct BookingsApi;

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetServicesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<ServiceOut>>),

    #[oai(status = 500)]
    InternalServerError,
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetAvailableMastersResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<MasterOut>>),

    #[oai(status = 500)]
    InternalServerError,
}

#[derive(Debug, Clone, Eq, PartialEq, ApiResponse)]
pub enum GetAvailableBranchesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<BranchOut>>),

    #[oai(status = 500)]
    InternalServerError,
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

#[OpenApi(prefix_path = "/bookings")]
impl BookingsApi {
    #[tracing::instrument]
    #[oai(path = "/services", method = "get")]
    async fn get_services(&self, Query(organization_id): Query<Uuid>) -> GetServicesResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/masters/available", method = "get")]
    async fn get_available_masters(
        &self,
        Query(organization_id): Query<Uuid>,
        Query(branches): Query<Vec<Uuid>>,
    ) -> GetAvailableMastersResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/branches/available", method = "get")]
    async fn get_available_branches(
        &self,
        Query(organization_id): Query<Uuid>,
        Query(masters): Query<Vec<Uuid>>,
    ) -> GetAvailableBranchesResponse {
        todo!()
    }
}
