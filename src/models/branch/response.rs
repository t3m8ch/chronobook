use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct BranchOut {
    id: Uuid,
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
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateBranchOut {
    id: Uuid,
}
