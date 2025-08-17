use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ServiceOut {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub duration_minutes: Option<u32>,
    pub price: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateServiceOut {
    pub id: Uuid,
}
