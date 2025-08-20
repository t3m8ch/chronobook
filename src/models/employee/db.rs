use chrono::NaiveDateTime;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telegram: Option<String>,
    pub is_owner: bool,
    pub is_manager: bool,
    pub is_master: bool,
    pub organization_id: Uuid,
    pub manager_branch_id: Option<Uuid>,
    pub user_id: Uuid,
}

#[derive(Debug, Clone, FromRow)]
pub struct EmployeeWithProfile {
    // Employee fields
    pub id: Uuid,
    pub employee_created_at: NaiveDateTime,
    pub employee_updated_at: NaiveDateTime,
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
    pub profile_created_at: NaiveDateTime,
    pub profile_updated_at: NaiveDateTime,
}
