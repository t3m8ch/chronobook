use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBranchRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub timezone: Option<String>,
    pub street: Option<String>,
    pub house_number: Option<String>,
    pub apartment_number: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true)]
    /// Address info. Send null to clear, omit to keep unchanged
    pub address_info: Option<Option<String>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBranchesQuery {
    pub organization_name: String,
    #[serde(default)]
    pub masters: Vec<Uuid>,
}
