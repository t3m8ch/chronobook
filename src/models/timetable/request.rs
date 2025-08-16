use chrono::{DateTime, NaiveDate, Utc};
use poem_openapi::{Enum, Object};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct CreateTimetableRequest {
    pub master_id: Uuid,
    pub recurrence_cycle_start: NaiveDate,
    pub schedule_days: Vec<ScheduleDayIn>,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct CreateDayRedefinitionRequest {
    pub master_id: Uuid,
    pub date: NaiveDate,
    pub schedule_day: ScheduleDayIn,
}

// Poem не поддерживает non-unit перечисления... Кринж
// Schemars используется в aide, и он поддерживает.
// TODO: Переписать проект на axum + aide
#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct ScheduleDayIn {
    branch_id: Option<Uuid>,
    working_interval: Option<Interval>,
    break_intervals: Option<Vec<Interval>>,
    day_type: ScheduleDayType,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct Interval {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Enum)]
pub enum ScheduleDayType {
    Weekday,
    Weekend,
}
