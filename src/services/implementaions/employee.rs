use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::employee::{
        common::EmployeeRole,
        request::{CreateEmployeeRequest, UpdateEmployeeRequest},
        response::{CreateEmployeeOut, EmployeeOut},
    },
    repositories::{auth::AuthRepository, employee::EmployeeRepository},
    services::{employee::EmployeeService, errors::ServiceError},
};

pub struct EmployeeServiceImpl {
    pool: PgPool,
    employee_repository: Arc<dyn EmployeeRepository>,
    auth_repository: Arc<dyn AuthRepository>,
}

impl EmployeeServiceImpl {
    pub fn new(
        pool: PgPool,
        employee_repository: Arc<dyn EmployeeRepository>,
        auth_repository: Arc<dyn AuthRepository>,
    ) -> Self {
        Self {
            pool,
            employee_repository,
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
                // For non-owner operations, any role in the organization is sufficient
                Ok(())
            }
            None => Err(ServiceError::Forbidden),
        }
    }

    fn convert_roles_to_flags(roles: &[EmployeeRole]) -> (bool, bool, Option<Uuid>) {
        let mut is_manager = false;
        let mut is_master = false;
        let mut manager_branch_id = None;

        for role in roles {
            match role {
                EmployeeRole::Manager { branch_id } => {
                    is_manager = true;
                    manager_branch_id = Some(*branch_id);
                }
                EmployeeRole::Master => {
                    is_master = true;
                }
            }
        }

        (is_manager, is_master, manager_branch_id)
    }

    fn convert_flags_to_roles(
        _is_owner: bool,
        is_manager: bool,
        is_master: bool,
        manager_branch_id: Option<Uuid>,
    ) -> Vec<EmployeeRole> {
        let mut roles = Vec::new();

        if is_manager {
            if let Some(branch_id) = manager_branch_id {
                roles.push(EmployeeRole::Manager { branch_id });
            }
        }

        if is_master {
            roles.push(EmployeeRole::Master);
        }

        roles
    }
}

#[async_trait]
impl EmployeeService for EmployeeServiceImpl {
    async fn create_employee(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        request: CreateEmployeeRequest,
    ) -> Result<CreateEmployeeOut, ServiceError> {
        // Check if the requesting user is an owner of the organization
        self.check_user_permission(user_id, organization_id, true)
            .await?;

        // Check if the user to be added as employee already exists as an employee in this org
        let existing = self
            .employee_repository
            .get_by_user_and_org(request.user_id, organization_id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        if existing.is_some() {
            return Err(ServiceError::ConflictError(
                "Employee already exists in this organization".to_string(),
            ));
        }

        // Convert roles to boolean flags
        let (is_manager, is_master, manager_branch_id) =
            Self::convert_roles_to_flags(&request.roles);

        // Start transaction
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(ServiceError::DatabaseError)?;

        // Create the employee
        let employee = self
            .employee_repository
            .create(
                &mut tx,
                organization_id,
                request.user_id,
                request.contact_phone,
                request.contact_email,
                request.contact_telegram,
                false, // is_owner - only the first employee (created during org creation) can be owner
                is_manager,
                is_master,
                manager_branch_id,
            )
            .await
            .map_err(ServiceError::DatabaseError)?;

        // Commit transaction
        tx.commit().await.map_err(ServiceError::DatabaseError)?;

        Ok(CreateEmployeeOut { id: employee.id })
    }

    async fn list_employees(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<EmployeeOut>, ServiceError> {
        // Check if user has any role in the organization
        self.check_user_permission(user_id, organization_id, false)
            .await?;

        // Get employees
        let employees = self
            .employee_repository
            .list(organization_id, limit, offset)
            .await
            .map_err(ServiceError::DatabaseError)?;

        // Convert to response DTOs
        Ok(employees
            .into_iter()
            .map(|emp| {
                let roles = Self::convert_flags_to_roles(
                    emp.is_owner,
                    emp.is_manager,
                    emp.is_master,
                    emp.manager_branch_id,
                );

                EmployeeOut {
                    id: emp.id,
                    contact_phone: emp.contact_phone.unwrap_or_default(),
                    contact_email: emp.contact_email.unwrap_or_default(),
                    contact_telegram: emp.contact_telegram.unwrap_or_default(),
                    roles,
                    organization_id: emp.organization_id,
                    user_id: emp.user_id,
                }
            })
            .collect())
    }

    async fn get_employee(
        &self,
        user_id: Uuid,
        employee_id: Uuid,
    ) -> Result<EmployeeOut, ServiceError> {
        // Get the employee
        let employee = self
            .employee_repository
            .get(employee_id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Employee not found".to_string()))?;

        // Check if requesting user has permission to view this employee
        self.check_user_permission(user_id, employee.organization_id, false)
            .await?;

        // Convert to response DTO
        let roles = Self::convert_flags_to_roles(
            employee.is_owner,
            employee.is_manager,
            employee.is_master,
            employee.manager_branch_id,
        );

        Ok(EmployeeOut {
            id: employee.id,
            contact_phone: employee.contact_phone.unwrap_or_default(),
            contact_email: employee.contact_email.unwrap_or_default(),
            contact_telegram: employee.contact_telegram.unwrap_or_default(),
            roles,
            organization_id: employee.organization_id,
            user_id: employee.user_id,
        })
    }

    async fn update_employee(
        &self,
        user_id: Uuid,
        employee_id: Uuid,
        request: UpdateEmployeeRequest,
    ) -> Result<EmployeeOut, ServiceError> {
        // Get the employee to check organization
        let current = self
            .employee_repository
            .get(employee_id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Employee not found".to_string()))?;

        // Check if requesting user is an owner of the organization
        self.check_user_permission(user_id, current.organization_id, true)
            .await?;

        // Convert roles to flags if provided
        let (is_manager, is_master, manager_branch_id) = if let Some(roles) = &request.roles {
            let (mgr, mstr, branch) = Self::convert_roles_to_flags(roles);
            (Some(mgr), Some(mstr), Some(branch))
        } else {
            (None, None, None)
        };

        // Update the employee
        let updated = self
            .employee_repository
            .update(
                employee_id,
                request.contact_phone,
                request.contact_email,
                request.contact_telegram,
                request.user_id,
                None, // is_owner cannot be changed via update
                is_manager,
                is_master,
                manager_branch_id,
            )
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Employee not found".to_string()))?;

        // Convert to response DTO
        let roles = Self::convert_flags_to_roles(
            updated.is_owner,
            updated.is_manager,
            updated.is_master,
            updated.manager_branch_id,
        );

        Ok(EmployeeOut {
            id: updated.id,
            contact_phone: updated.contact_phone.unwrap_or_default(),
            contact_email: updated.contact_email.unwrap_or_default(),
            contact_telegram: updated.contact_telegram.unwrap_or_default(),
            roles,
            organization_id: updated.organization_id,
            user_id: updated.user_id,
        })
    }

    async fn delete_employee(&self, user_id: Uuid, employee_id: Uuid) -> Result<(), ServiceError> {
        // Get the employee to check organization
        let employee = self
            .employee_repository
            .get(employee_id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Employee not found".to_string()))?;

        // Check if requesting user is an owner of the organization
        self.check_user_permission(user_id, employee.organization_id, true)
            .await?;

        // Prevent deleting the owner
        if employee.is_owner {
            return Err(ServiceError::ValidationError(
                "Cannot delete the organization owner".to_string(),
            ));
        }

        // Delete the employee
        let deleted = self
            .employee_repository
            .delete(employee_id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        if !deleted {
            return Err(ServiceError::NotFound("Employee not found".to_string()));
        }

        Ok(())
    }
}
