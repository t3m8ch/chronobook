use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct OrganizationDashboard {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub has_branch: bool,
    pub has_master: bool,
    pub has_timetable: bool,
    pub has_service: bool,
}