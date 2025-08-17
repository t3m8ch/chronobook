use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimetableRequest {
    pub master_id: Uuid,
    pub recurrence_cycle_start: NaiveDate,
    pub schedule_days: Vec<ScheduleDayIn>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTimetableRequest {
    pub recurrence_cycle_start: Option<NaiveDate>,
    pub schedule_days: Option<Vec<ScheduleDayIn>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateDayRedefinitionRequest {
    pub master_id: Uuid,
    pub date: NaiveDate,
    pub schedule_day: ScheduleDayIn,
}

// TODO: refactor this
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleDayIn {
    pub branch_id: Option<Uuid>,
    pub working_interval: Option<Interval>,
    pub break_intervals: Option<Vec<Interval>>,
    pub day_type: ScheduleDayType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ScheduleDayType {
    Weekday,
    Weekend,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowsQuery {
    pub organization_name: String,
    pub masters: Vec<Uuid>,
    pub branches: Vec<Uuid>,
    pub min_datetime: DateTime<Utc>,
    pub max_datetime: DateTime<Utc>,
}
