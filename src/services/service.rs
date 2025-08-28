use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    models::service::{
        request::{CreateServiceRequest, UpdateServiceRequest},
        response::{CreateServiceOut, ServiceOut},
    },
    services::errors::ServiceError,
};

#[async_trait]
pub trait ServiceService: Send + Sync {
    async fn create_service(
        &self,
        request: CreateServiceRequest,
        organization_id: Uuid,
    ) -> Result<CreateServiceOut, ServiceError>;

    async fn list_services(
        &self,
        organization_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ServiceOut>, ServiceError>;

    async fn get_service(
        &self,
        id: Uuid,
        organization_id: Uuid,
    ) -> Result<ServiceOut, ServiceError>;

    async fn update_service(
        &self,
        id: Uuid,
        request: UpdateServiceRequest,
        organization_id: Uuid,
    ) -> Result<ServiceOut, ServiceError>;

    async fn delete_service(&self, id: Uuid, organization_id: Uuid) -> Result<(), ServiceError>;

    async fn find_organization_by_service(
        &self,
        service_id: Uuid,
    ) -> Result<Option<Uuid>, ServiceError>;
}
