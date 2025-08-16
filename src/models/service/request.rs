use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceRequest {
    pub display_name: String,
    pub description: String,
    pub duration_minutes: Option<i32>,
    pub price: String,
    pub master_id: Option<Uuid>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetServicesQuery {
    pub organization_id: Uuid,
}
