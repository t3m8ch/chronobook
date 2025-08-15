use poem_openapi::{OpenApi, param::Path, payload::Json};
use uuid::Uuid;

use crate::models::{
    branch::{request::CreateBranchRequest, response::CreateBranchResponse},
    dashboard::response::GetOrganizationDashboardResponse,
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
}
