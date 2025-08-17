use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateBranchRequest {
    pub name: String,
    pub description: String,
    pub timezone: String,
    pub street: String,
    pub house_number: String,
    pub apartment_number: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub address_info: Option<String>,
    pub organization_id: Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetBranchesQuery {
    pub organization_name: Uuid,
    #[serde(default)]
    pub masters: Vec<Uuid>,
}
