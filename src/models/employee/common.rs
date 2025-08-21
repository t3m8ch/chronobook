use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase", tag = "role")]
#[garde(context(()))]
pub enum EmployeeRole {
    Manager {
        #[garde(skip)]
        branch_id: Uuid,
    },
    Master,
}
