use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ServiceOut {
    id: Uuid,
    name: String,
    description: String,
    duration_minutes: Option<u32>,
    price: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceOut {
    pub id: Uuid,
}
