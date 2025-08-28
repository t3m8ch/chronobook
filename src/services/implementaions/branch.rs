use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    models::branch::{
        request::{CreateBranchRequest, UpdateBranchRequest},
        response::{BranchOut, CreateBranchOut},
    },
    repositories::{auth::AuthRepository, branch::BranchRepository},
    services::{branch::BranchService, errors::ServiceError},
};

pub struct BranchServiceImpl {
    branch_repository: Arc<dyn BranchRepository>,
    auth_repository: Arc<dyn AuthRepository>,
}

impl BranchServiceImpl {
    pub fn new(
        branch_repository: Arc<dyn BranchRepository>,
        auth_repository: Arc<dyn AuthRepository>,
    ) -> Self {
        Self {
            branch_repository,
            auth_repository,
        }
    }

    async fn check_user_permission(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        require_owner: bool,
    ) -> Result<(), ServiceError> {
        // Check if user is an employee of the organization
        let employee = self
            .auth_repository
            .find_employee_by_user_and_org(user_id, organization_id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        match employee {
            Some(emp) => {
                if require_owner && !emp.is_owner {
                    return Err(ServiceError::Forbidden);
                }
                Ok(())
            }
            None => Err(ServiceError::Forbidden),
        }
    }

    async fn check_branch_permission(
        &self,
        user_id: Uuid,
        branch_id: Uuid,
        require_owner: bool,
        allow_manager: bool,
    ) -> Result<Uuid, ServiceError> {
        // Get the organization ID from the branch
        let org_id = self
            .branch_repository
            .get_organization_id(branch_id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Branch not found".to_string()))?;

        // Check if user is an employee of the organization
        let employee = self
            .auth_repository
            .find_employee_by_user_and_org(user_id, org_id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::Forbidden)?;

        // Check permissions
        if require_owner && !employee.is_owner {
            return Err(ServiceError::Forbidden);
        }

        if allow_manager && employee.is_manager {
            // Managers can only manage their assigned branch
            if let Some(manager_branch_id) = employee.manager_branch_id {
                if manager_branch_id != branch_id {
                    return Err(ServiceError::Forbidden);
                }
            } else {
                return Err(ServiceError::Forbidden);
            }
        } else if !employee.is_owner {
            return Err(ServiceError::Forbidden);
        }

        Ok(org_id)
    }
}

#[async_trait]
impl BranchService for BranchServiceImpl {
    async fn create_branch(
        &self,
        user_id: Uuid,
        request: CreateBranchRequest,
    ) -> Result<CreateBranchOut, ServiceError> {
        // Check if user has permission to create branches in this organization
        self.check_user_permission(user_id, request.organization_id, true)
            .await?;

        // Create the branch
        let branch = self
            .branch_repository
            .create(
                request.name,
                request.description,
                request.timezone,
                request.street,
                request.house_number,
                request.apartment_number,
                request.city,
                request.region,
                request.country,
                request.address_info,
                request.organization_id,
            )
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(CreateBranchOut { id: branch.id })
    }

    async fn list_branches(
        &self,
        user_id: Uuid,
        organization_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BranchOut>, ServiceError> {
        // If organization_id is provided, check permissions for that org
        // Otherwise, this would need to be handled differently based on requirements
        if let Some(org_id) = organization_id {
            // Check if user has any role in the organization
            let employee = self
                .auth_repository
                .find_employee_by_user_and_org(user_id, org_id)
                .await
                .map_err(ServiceError::DatabaseError)?;

            if employee.is_none() {
                return Err(ServiceError::Forbidden);
            }
        }

        // Get branches
        let branches = self
            .branch_repository
            .list(organization_id, limit, offset)
            .await
            .map_err(ServiceError::DatabaseError)?;

        // Convert to response DTOs
        Ok(branches
            .into_iter()
            .map(|branch| BranchOut {
                id: branch.id,
                name: branch.display_name,
                description: branch.description,
                timezone: branch.timezone,
                street: branch.street,
                house_number: branch.house_number,
                apartment_number: branch.apartment_number.unwrap_or_default(),
                city: branch.city,
                region: branch.region,
                country: branch.country,
                address_info: branch.address_info,
            })
            .collect())
    }

    async fn get_branch(&self, user_id: Uuid, branch_id: Uuid) -> Result<BranchOut, ServiceError> {
        // Get the branch
        let branch = self
            .branch_repository
            .get(branch_id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Branch not found".to_string()))?;

        // Check if user has permission to view this branch
        let employee = self
            .auth_repository
            .find_employee_by_user_and_org(user_id, branch.organization_id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        if employee.is_none() {
            return Err(ServiceError::Forbidden);
        }

        // Convert to response DTO
        Ok(BranchOut {
            id: branch.id,
            name: branch.display_name,
            description: branch.description,
            timezone: branch.timezone,
            street: branch.street,
            house_number: branch.house_number,
            apartment_number: branch.apartment_number.unwrap_or_default(),
            city: branch.city,
            region: branch.region,
            country: branch.country,
            address_info: branch.address_info,
        })
    }

    async fn update_branch(
        &self,
        user_id: Uuid,
        branch_id: Uuid,
        request: UpdateBranchRequest,
    ) -> Result<BranchOut, ServiceError> {
        // Check permissions (owners and managers of the branch can update)
        self.check_branch_permission(user_id, branch_id, false, true)
            .await?;

        // Update the branch
        let branch = self
            .branch_repository
            .update(
                branch_id,
                request.name,
                request.description,
                request.timezone,
                request.street,
                request.house_number,
                request.apartment_number,
                request.city,
                request.region,
                request.country,
                request.address_info,
            )
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Branch not found".to_string()))?;

        // Convert to response DTO
        Ok(BranchOut {
            id: branch.id,
            name: branch.display_name,
            description: branch.description,
            timezone: branch.timezone,
            street: branch.street,
            house_number: branch.house_number,
            apartment_number: branch.apartment_number.unwrap_or_default(),
            city: branch.city,
            region: branch.region,
            country: branch.country,
            address_info: branch.address_info,
        })
    }

    async fn delete_branch(&self, user_id: Uuid, branch_id: Uuid) -> Result<(), ServiceError> {
        // Check permissions (only owners can delete)
        self.check_branch_permission(user_id, branch_id, true, false)
            .await?;

        // Delete the branch
        let deleted = self
            .branch_repository
            .delete(branch_id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        if !deleted {
            return Err(ServiceError::NotFound("Branch not found".to_string()));
        }

        Ok(())
    }
}
