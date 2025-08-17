use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmployeeOut {
    pub id: Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeOut {
    pub id: Uuid,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: Option<String>,
    pub organization_id: Uuid,
    pub is_owner: bool,
    pub is_manager: bool,
    pub is_master: bool,
}
