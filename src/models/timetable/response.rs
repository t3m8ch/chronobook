use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::{branch::response::BranchOut, master::response::MasterOut};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimetableOut {
    pub master_id: Uuid,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TimetableOut {
    pub master_id: Uuid,
    pub recurrence_cycle_start: NaiveDate,
    pub recurrence_cycle_days: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleDayOut {
    pub master_id: Uuid,
    pub day_ordinal: i32,
    pub day_data: Value,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DayRedefinitionOut {
    pub master_id: Uuid,
    pub date: NaiveDate,
    pub day_data: Value,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WindowOut {
    pub id: Uuid,
    pub slots: Vec<(DateTime<Utc>, DateTime<Utc>)>,
    pub master: MasterOut,
    pub branch: BranchOut,
}
