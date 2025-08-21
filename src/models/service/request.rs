use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceRequest {
    pub display_name: String,
    pub description: String,
    pub duration_minutes: i32,
    pub price: Option<String>,
    pub master_id: Option<Uuid>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceRequest {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub duration_minutes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true)]
    /// Price. Send null to clear, omit to keep unchanged
    pub price: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true)]
    /// Master ID. Send null to clear, omit to keep unchanged
    pub master_id: Option<Option<Uuid>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetServicesQuery {
    #[garde(skip)]
    pub organization_name: String,

    #[garde(skip)]
    #[serde(default)]
    pub masters: Vec<Uuid>,
}
