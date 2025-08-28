use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    models::service::{
        db::Service,
        request::{CreateServiceRequest, UpdateServiceRequest},
        response::{CreateServiceOut, ServiceOut},
    },
    repositories::service::ServiceRepository,
    services::{errors::ServiceError, service::ServiceService},
};

pub struct ServiceServiceImpl {
    service_repository: Arc<dyn ServiceRepository>,
}

impl ServiceServiceImpl {
    pub fn new(service_repository: Arc<dyn ServiceRepository>) -> Self {
        Self { service_repository }
    }

    fn service_to_out(&self, service: Service) -> ServiceOut {
        ServiceOut {
            id: service.id,
            name: service.display_name,
            description: service.description,
            duration_minutes: service.duration_minutes as u32,
            price: service.price.map(|p| p.to_string()),
        }
    }
}

#[async_trait]
impl ServiceService for ServiceServiceImpl {
    async fn create_service(
        &self,
        request: CreateServiceRequest,
        _organization_id: Uuid,
    ) -> Result<CreateServiceOut, ServiceError> {
        // If master_id is specified, verify it belongs to the organization
        if let Some(_master_id) = request.master_id {
            // TODO: Need to add master organization validation
            // We would need to add a method to validate master belongs to organization
        }

        let service_id = self
            .service_repository
            .create_service(
                request.display_name,
                request.description,
                request.duration_minutes,
                request.price,
                request.master_id,
            )
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(CreateServiceOut { id: service_id })
    }

    async fn list_services(
        &self,
        organization_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ServiceOut>, ServiceError> {
        let services = self
            .service_repository
            .find_services_by_organization(organization_id, limit, offset)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(services
            .into_iter()
            .map(|s| self.service_to_out(s))
            .collect())
    }

    async fn get_service(
        &self,
        id: Uuid,
        organization_id: Uuid,
    ) -> Result<ServiceOut, ServiceError> {
        let service = self
            .service_repository
            .find_service_by_id(id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Service not found".to_string()))?;

        // Check organization access
        if service.organization_id != Some(organization_id) {
            return Err(ServiceError::NotFound("Service not found".to_string())); // Return NotFound instead of Forbidden for security
        }

        Ok(self.service_to_out(service))
    }

    async fn update_service(
        &self,
        id: Uuid,
        request: UpdateServiceRequest,
        organization_id: Uuid,
    ) -> Result<ServiceOut, ServiceError> {
        // First check if service exists and belongs to organization
        let existing_service = self
            .service_repository
            .find_service_by_id(id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Service not found".to_string()))?;

        if existing_service.organization_id != Some(organization_id) {
            return Err(ServiceError::NotFound("Service not found".to_string()));
        }

        let service = self
            .service_repository
            .update_service(
                id,
                request.display_name,
                request.description,
                request.duration_minutes,
                request.price,
                request.master_id,
            )
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ServiceError::NotFound("Service not found".to_string()),
                _ => ServiceError::DatabaseError(e),
            })?;

        Ok(self.service_to_out(service))
    }

    async fn delete_service(&self, id: Uuid, organization_id: Uuid) -> Result<(), ServiceError> {
        // First check if service exists and belongs to organization
        let existing_service = self
            .service_repository
            .find_service_by_id(id)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or(ServiceError::NotFound("Service not found".to_string()))?;

        if existing_service.organization_id != Some(organization_id) {
            return Err(ServiceError::NotFound("Service not found".to_string()));
        }

        let deleted = self
            .service_repository
            .delete_service(id)
            .await
            .map_err(ServiceError::DatabaseError)?;

        if !deleted {
            return Err(ServiceError::NotFound("Service not found".to_string()));
        }

        Ok(())
    }

    async fn find_organization_by_service(
        &self,
        service_id: Uuid,
    ) -> Result<Option<Uuid>, ServiceError> {
        self.service_repository
            .find_organization_id_by_service(service_id)
            .await
            .map_err(ServiceError::DatabaseError)
    }
}
