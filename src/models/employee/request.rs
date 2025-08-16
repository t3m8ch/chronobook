use poem_openapi::Object;
use uuid::Uuid;

use crate::models::validation::PhoneValidator;

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct CreateEmployeeRequest {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub manager_branch_id: Option<Uuid>,
    #[oai(validator(custom = "PhoneValidator"))]
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telegram: Option<String>,
    pub is_owner: bool,
    pub is_manager: bool,
    pub is_master: bool,
}
