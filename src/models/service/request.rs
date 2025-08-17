use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceRequest {
    pub display_name: String,
    pub description: String,
    pub duration_minutes: Option<i32>,
    pub price: String,
    pub master_id: Option<Uuid>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceRequest {
    pub display_name: Option<String>,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true)]
    /// Duration in minutes. Send null to clear, omit to keep unchanged
    pub duration_minutes: Option<Option<i32>>,
    pub price: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true)]
    /// Master ID. Send null to clear, omit to keep unchanged
    pub master_id: Option<Option<Uuid>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServicesQuery {
    pub organization_name: String,
}
