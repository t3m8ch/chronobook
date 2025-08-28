use async_trait::async_trait;
use uuid::Uuid;

use crate::models::employee::{
    request::{CreateEmployeeRequest, UpdateEmployeeRequest},
    response::{CreateEmployeeOut, EmployeeOut},
};

use super::errors::ServiceError;

#[mockall::automock]
#[async_trait]
pub trait EmployeeService: Send + Sync {
    async fn create_employee(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        request: CreateEmployeeRequest,
    ) -> Result<CreateEmployeeOut, ServiceError>;

    async fn list_employees(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<EmployeeOut>, ServiceError>;

    async fn get_employee(
        &self,
        user_id: Uuid,
        employee_id: Uuid,
    ) -> Result<EmployeeOut, ServiceError>;

    async fn update_employee(
        &self,
        user_id: Uuid,
        employee_id: Uuid,
        request: UpdateEmployeeRequest,
    ) -> Result<EmployeeOut, ServiceError>;

    async fn delete_employee(&self, user_id: Uuid, employee_id: Uuid) -> Result<(), ServiceError>;
}
