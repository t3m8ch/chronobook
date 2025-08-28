use async_trait::async_trait;
use uuid::Uuid;

use crate::models::organization::db::Organization;
use crate::models::organization::request::CreateOrganizationRequest;
use crate::services::errors::ServiceError;

#[async_trait]
pub trait OrganizationService: Send + Sync {
    async fn create_organization(
        &self,
        user_id: Uuid,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, ServiceError>;
}
