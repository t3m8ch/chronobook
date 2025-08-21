use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateBranchRequest {
    #[garde(skip)]
    pub name: String,

    #[garde(skip)]
    pub description: String,

    #[garde(skip)]
    pub timezone: String,

    #[garde(skip)]
    pub street: String,

    #[garde(skip)]
    pub house_number: String,

    #[garde(skip)]
    pub apartment_number: Option<String>,

    #[garde(skip)]
    pub city: String,

    #[garde(skip)]
    pub region: String,

    #[garde(skip)]
    pub country: String,

    #[garde(skip)]
    pub address_info: Option<String>,

    #[garde(skip)]
    pub organization_id: Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBranchRequest {
    #[garde(skip)]
    pub name: Option<String>,

    #[garde(skip)]
    pub description: Option<String>,

    #[garde(skip)]
    pub timezone: Option<String>,

    #[garde(skip)]
    pub street: Option<String>,

    #[garde(skip)]
    pub house_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true)]
    #[garde(skip)]
    /// Apartment number. Send null to clear, omit to keep unchanged
    pub apartment_number: Option<Option<String>>,

    #[garde(skip)]
    pub city: Option<String>,

    #[garde(skip)]
    pub region: Option<String>,

    #[garde(skip)]
    pub country: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true)]
    #[garde(skip)]
    /// Address info. Send null to clear, omit to keep unchanged
    pub address_info: Option<Option<String>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetBranchesQuery {
    #[garde(skip)]
    pub organization_name: String,

    #[garde(skip)]
    #[serde(default)]
    pub masters: Vec<Uuid>,
}
