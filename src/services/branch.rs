use async_trait::async_trait;
use uuid::Uuid;

use crate::models::branch::{
    request::{CreateBranchRequest, UpdateBranchRequest},
    response::{BranchOut, CreateBranchOut},
};

use super::errors::ServiceError;

#[mockall::automock]
#[async_trait]
pub trait BranchService: Send + Sync {
    async fn create_branch(
        &self,
        user_id: Uuid,
        request: CreateBranchRequest,
    ) -> Result<CreateBranchOut, ServiceError>;

    async fn list_branches(
        &self,
        user_id: Uuid,
        organization_id: Option<Uuid>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BranchOut>, ServiceError>;

    async fn get_branch(&self, user_id: Uuid, branch_id: Uuid) -> Result<BranchOut, ServiceError>;

    async fn update_branch(
        &self,
        user_id: Uuid,
        branch_id: Uuid,
        request: UpdateBranchRequest,
    ) -> Result<BranchOut, ServiceError>;

    async fn delete_branch(&self, user_id: Uuid, branch_id: Uuid) -> Result<(), ServiceError>;
}
