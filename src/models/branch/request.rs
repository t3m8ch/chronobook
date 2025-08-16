use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateBranchRequest {
    name: String,
    description: String,
    timezone: String,
    street: String,
    house_number: String,
    apartment_number: String,
    city: String,
    region: String,
    country: String,
    address_info: Option<String>,
    organization_id: Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetBranchesQuery {
    pub organization_id: Uuid,
    #[serde(default)]
    pub masters: Vec<Uuid>,
}
