use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceRequest {
    #[garde(skip)]
    pub display_name: String,

    #[garde(skip)]
    pub description: String,

    #[garde(skip)]
    pub duration_minutes: i32,

    #[garde(skip)]
    pub price: Option<String>,

    #[garde(skip)]
    pub master_id: Option<Uuid>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceRequest {
    #[garde(skip)]
    pub display_name: Option<String>,

    #[garde(skip)]
    pub description: Option<String>,

    #[garde(skip)]
    pub duration_minutes: Option<i32>,

    #[garde(skip)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true)]
    /// Price. Send null to clear, omit to keep unchanged
    pub price: Option<Option<String>>,

    #[garde(skip)]
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
