use poem_openapi::{OpenApi, param::Query};
use uuid::Uuid;

use crate::models::{
    branch::response::GetAdminBranchesResponse, master::response::GetAdminMastersResponse,
};

#[derive(Clone, Debug)]
pub struct AdminApi;

#[OpenApi(prefix_path = "/admin")]
impl AdminApi {
    #[tracing::instrument]
    #[oai(path = "/branches", method = "get")]
    async fn get_branches(&self, Query(organization_id): Query<Uuid>) -> GetAdminBranchesResponse {
        todo!()
    }

    #[tracing::instrument]
    #[oai(path = "/masters", method = "get")]
    async fn get_masters(&self, Query(organization_id): Query<Uuid>) -> GetAdminMastersResponse {
        todo!()
    }
}
