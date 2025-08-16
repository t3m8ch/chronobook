use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateTimetableRequest {
    pub master_id: Uuid,
    pub recurrence_cycle_start: NaiveDate,
    pub schedule_days: Vec<ScheduleDayIn>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateDayRedefinitionRequest {
    pub master_id: Uuid,
    pub date: NaiveDate,
    pub schedule_day: ScheduleDayIn,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ScheduleDayIn {
    branch_id: Option<Uuid>,
    working_interval: Option<Interval>,
    break_intervals: Option<Vec<Interval>>,
    day_type: ScheduleDayType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Interval {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum ScheduleDayType {
    Weekday,
    Weekend,
}
