use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telegram: Option<String>,
    pub is_owner: bool,
    pub is_manager: bool,
    pub manager_branch_id: Option<Uuid>,
}

#[derive(Debug, Clone, FromRow)]
pub struct EmployeeWithProfile {
    // Employee fields
    pub id: Uuid,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telegram: Option<String>,
    pub is_owner: bool,
    pub is_manager: bool,
    pub is_master: bool,
    pub organization_id: Uuid,
    pub manager_branch_id: Option<Uuid>,
    pub user_id: Uuid,
    // UserProfile fields
    pub first_name: String,
    pub last_name: String,
    pub patronymic: Option<String>,
}
