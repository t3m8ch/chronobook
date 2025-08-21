use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationRequest {
    #[garde(length(min = 3, max = 50), pattern(r"^[a-z0-9_-]+$"))]
    pub name: String,

    #[garde(length(min = 1, max = 100))]
    pub display_name: String,

    #[garde(length(max = 500))]
    pub description: Option<String>,
}
