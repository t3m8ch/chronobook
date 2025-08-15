use poem_openapi::{OpenApi, param::Path};
use uuid::Uuid;

use crate::models::dashboard::response::GetOrganizationDashboardResponse;

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
}
