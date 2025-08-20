use chrono::{NaiveDate, NaiveDateTime};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Timetable {
    pub master_id: Uuid,
    pub recurrence_cycle_start: NaiveDate,
}

#[derive(Debug, Clone, FromRow)]
pub struct ScheduleDay {
    pub master_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub day_ordinal: i32,
    pub day_data: Value,
}

#[derive(Debug, Clone, FromRow)]
pub struct DayRedefinition {
    pub master_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub date: NaiveDate,
    pub day_data: Value,
}
