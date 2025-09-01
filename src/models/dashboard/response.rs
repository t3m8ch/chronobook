use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationDashboardOut {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub active: bool,
    pub al_least_one_branch: bool,
    pub al_least_one_master: bool,
    pub al_least_one_timetable: bool,
    pub al_least_one_service: bool,
}
