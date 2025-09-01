use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    models::dashboard::response::OrganizationDashboardOut,
    repositories::dashboard::DashboardRepository, services::errors::ServiceError,
};

#[async_trait]
pub trait DashboardService: Send + Sync {
    async fn get_organization_dashboard(
        &self,
        organization_id: Uuid,
    ) -> Result<OrganizationDashboardOut, ServiceError>;
}

pub struct DashboardServiceImpl {
    dashboard_repository: Arc<dyn DashboardRepository>,
}

impl DashboardServiceImpl {
    pub fn new(dashboard_repository: Arc<dyn DashboardRepository>) -> Self {
        Self {
            dashboard_repository,
        }
    }
}

#[async_trait]
impl DashboardService for DashboardServiceImpl {
    async fn get_organization_dashboard(
        &self,
        organization_id: Uuid,
    ) -> Result<OrganizationDashboardOut, ServiceError> {
        let dashboard = self
            .dashboard_repository
            .get_organization_dashboard(organization_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ServiceError::NotFound(format!(
                    "Organization with id {} not found",
                    organization_id
                )),
                _ => ServiceError::DatabaseError(e),
            })?;

        Ok(dashboard)
    }
}
