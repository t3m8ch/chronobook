use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BookingOut {
    pub id: Uuid,
    pub service_id: Uuid,
    pub master_id: Uuid,
    pub branch_id: Uuid,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
