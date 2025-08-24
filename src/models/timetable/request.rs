use chrono::{NaiveDate, NaiveDateTime};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTimetableRequest {
    #[garde(skip)]
    pub master_id: Uuid,
    #[garde(skip)]
    pub recurrence_cycle_start: NaiveDate,
    #[garde(length(min = 1, max = 7))]
    pub schedule_days: Vec<ScheduleDayIn>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTimetableRequest {
    #[garde(skip)]
    pub recurrence_cycle_start: Option<NaiveDate>,
    #[garde(dive)]
    pub schedule_days: Option<Vec<ScheduleDayIn>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateDayRedefinitionRequest {
    #[garde(skip)]
    pub master_id: Uuid,
    #[garde(skip)]
    pub date: NaiveDate,
    #[garde(dive)]
    pub schedule_day: ScheduleDayIn,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase", tag = "dayType")]
pub enum ScheduleDayIn {
    Weekday {
        #[garde(skip)]
        branch_id: Uuid,
        #[garde(dive)]
        working_interval: Interval,
        #[garde(dive)]
        break_intervals: Vec<Interval>,
    },
    Weekend,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    #[garde(skip)]
    pub start: NaiveDateTime,
    #[garde(skip)]
    pub end: NaiveDateTime,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowsQuery {
    #[garde(skip)]
    pub organization_name: String,

    #[garde(skip)]
    pub service_id: Uuid,

    #[garde(skip)]
    pub masters: Vec<Uuid>,

    #[garde(skip)]
    pub branches: Vec<Uuid>,

    #[garde(skip)]
    pub min_datetime: NaiveDateTime,

    #[garde(skip)]
    pub max_datetime: NaiveDateTime,
}
