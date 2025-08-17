use crate::models::validation::validate_phone;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[garde(context(()))]
pub struct CreateEmployeeRequest {
    #[garde(custom(validate_phone))]
    #[schema(example = "+1234567890")]
    pub phone: String,

    #[garde(length(min = 1, max = 100))]
    #[schema(example = "John")]
    pub first_name: String,

    #[garde(length(min = 1, max = 100))]
    #[schema(example = "Doe")]
    pub last_name: String,

    #[garde(length(min = 1, max = 100))]
    #[schema(example = "Smith")]
    pub patronymic: Option<String>,

    #[garde(skip)]
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub organization_id: Uuid,

    #[garde(skip)]
    #[schema(example = false)]
    pub is_owner: bool,

    #[garde(skip)]
    #[schema(example = false)]
    pub is_manager: bool,

    #[garde(skip)]
    #[schema(example = true)]
    pub is_master: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmployeeRequest {
    #[schema(example = "+1234567890")]
    pub phone: Option<String>,

    #[schema(example = "John")]
    pub first_name: Option<String>,

    #[schema(example = "Doe")]
    pub last_name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true, example = "Smith")]
    /// Patronymic. Send null to clear, omit to keep unchanged
    pub patronymic: Option<Option<String>>,

    #[schema(example = false)]
    pub is_owner: Option<bool>,

    #[schema(example = false)]
    pub is_manager: Option<bool>,

    #[schema(example = true)]
    pub is_master: Option<bool>,
}
