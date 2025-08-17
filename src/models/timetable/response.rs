use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::{branch::response::BranchOut, master::response::MasterOut};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateTimetableOut {
    pub master_id: Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct WindowOut {
    pub id: Uuid,
    pub slots: Vec<(DateTime<Utc>, DateTime<Utc>)>,
    pub master: MasterOut,
    pub branch: BranchOut,
}
