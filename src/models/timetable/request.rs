use chrono::{NaiveDate, NaiveDateTime};
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase", tag = "dayType")]
pub enum ScheduleDayIn {
    Weekday {
        branch_id: Uuid,
        working_interval: Interval,
        break_intervals: Vec<Interval>,
    },
    Weekend,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowsQuery {
    pub organization_name: String,
    pub masters: Vec<Uuid>,
    pub branches: Vec<Uuid>,
    pub min_datetime: NaiveDateTime,
    pub max_datetime: NaiveDateTime,
}
