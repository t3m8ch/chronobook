use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::employee::common::EmployeeRole;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmployeeOut {
    pub id: Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeOut {
    pub id: Uuid,
    pub contact_phone: String,
    pub contact_email: String,
    pub contact_telegram: String,
    pub roles: Vec<EmployeeRole>,
    pub organization_id: Uuid,
    pub user_id: Uuid,
}
