use poem_openapi::{OpenApi, param::Path, payload::Json};
use uuid::Uuid;

use crate::models::{
    branch::{request::CreateBranchRequest, response::CreateBranchResponse},
    dashboard::response::GetOrganizationDashboardResponse,
    employee::{request::CreateEmployeeRequest, response::CreateEmployeeResponse},
    service::{request::CreateServiceRequest, response::CreateServiceResponse},
    timetable::{
        request::{CreateDayRedefinitionRequest, CreateTimetableRequest},
        response::{CreateDayRedefinitionResponse, CreateTimetableResponse},
    },
};

#[derive(Clone, Debug)]
pub struct AdminApi;

#[OpenApi(prefix_path = "/admin")]
impl AdminApi {
    #[tracing::instrument]
    #[oai(path = "/dashboard/{organization_id}", method = "get")]
    pub async fn get_organization_dashboard(
        &self,
        Path(organization_id): Path<Uuid>,
    ) -> GetOrganizationDashboardResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/branches", method = "post")]
    pub async fn create_branch(
        &self,
        Json(request): Json<CreateBranchRequest>,
    ) -> CreateBranchResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/employees", method = "post")]
    pub async fn create_employee(
        &self,
        Json(request): Json<CreateEmployeeRequest>,
    ) -> CreateEmployeeResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/employees/timetables", method = "post")]
    pub async fn create_employee_timetable(
        &self,
        Json(request): Json<CreateTimetableRequest>,
    ) -> CreateTimetableResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/employees/timetables/redefinitions", method = "post")]
    pub async fn create_day_redefinition(
        &self,
        Json(request): Json<CreateDayRedefinitionRequest>,
    ) -> CreateDayRedefinitionResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/services", method = "post")]
    pub async fn create_service(
        &self,
        Json(request): Json<CreateServiceRequest>,
    ) -> CreateServiceResponse {
        todo!()
    }
}
