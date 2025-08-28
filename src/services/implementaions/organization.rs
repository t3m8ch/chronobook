use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::organization::{db::Organization, request::CreateOrganizationRequest},
    repositories::{employee::EmployeeRepository, organization::OrganizationRepository},
    services::{errors::ServiceError, organization::OrganizationService},
};

pub struct OrganizationServiceImpl {
    pool: PgPool,
    organization_repo: Arc<dyn OrganizationRepository>,
    employee_repo: Arc<dyn EmployeeRepository>,
}

impl OrganizationServiceImpl {
    pub fn new(
        pool: PgPool,
        organization_repo: Arc<dyn OrganizationRepository>,
        employee_repo: Arc<dyn EmployeeRepository>,
    ) -> Self {
        Self {
            pool,
            organization_repo,
            employee_repo,
        }
    }
}

#[async_trait]
impl OrganizationService for OrganizationServiceImpl {
    async fn create_organization(
        &self,
        user_id: Uuid,
        request: CreateOrganizationRequest,
    ) -> Result<Organization, ServiceError> {
        // Check if organization name already exists
        if self
            .organization_repo
            .organization_exists_by_name(&request.name)
            .await?
        {
            return Err(ServiceError::ConflictError(format!(
                "Organization with name '{}' already exists",
                request.name
            )));
        }

        // Start transaction
        let mut tx = self.pool.begin().await?;

        // Create organization
        let organization = self
            .organization_repo
            .create_organization(
                &mut tx,
                &request.name,
                &request.display_name,
                request.description.as_deref(),
            )
            .await?;

        // Create employee record with owner role
        self.employee_repo
            .create(
                organization.id,
                user_id,
                None,  // contact_phone
                None,  // contact_email
                None,  // contact_telegram
                true,  // is_owner
                false, // is_manager
                false, // is_master
                None,  // manager_branch_id
            )
            .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(organization)
    }
}
