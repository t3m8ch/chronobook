use crate::models::validation::validate_phone;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
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
