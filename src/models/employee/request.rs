use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::employee::common::EmployeeRole;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
#[garde(context(()))]
pub struct CreateEmployeeRequest {
    #[garde(phone_number)]
    #[schema(example = "+1234567890")]
    pub contact_phone: Option<String>,

    #[garde(email)]
    #[schema(example = "john.doe@example.com")]
    pub contact_email: Option<String>,

    #[garde(skip)]
    #[schema(example = "durov")]
    pub contact_telegram: Option<String>,

    #[garde(skip)]
    pub user_id: Uuid,

    #[garde(skip)]
    pub roles: Vec<EmployeeRole>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmployeeRequest {
    #[garde(phone_number)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true, example = "+1234567890")]
    pub contact_phone: Option<Option<String>>,

    #[garde(email)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true, example = "john.doe@example.com")]
    pub contact_email: Option<Option<String>>,

    #[garde(skip)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schema(nullable = true, example = "durov")]
    pub contact_telegram: Option<Option<String>>,

    #[garde(skip)]
    pub user_id: Option<Uuid>,

    #[garde(skip)]
    pub roles: Option<Vec<EmployeeRole>>,
}
