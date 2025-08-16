use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct OrganizationOut {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
}
