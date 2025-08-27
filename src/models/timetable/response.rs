use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    models::{branch::response::BranchOut, master::response::MasterOut},
    services::booking::{DayData, Interval, Window},
};

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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TimetableWithRedefinitionsOut {
    pub timetable: TimetableOut,
    pub schedule_days: Vec<ScheduleDayOut>,
    pub redefinitions: Vec<DayRedefinitionOut>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleDayOut {
    pub master_id: Uuid,
    pub day_ordinal: i32,
    pub day_data: DayDataOut,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DayRedefinitionOut {
    pub master_id: Uuid,
    pub date: NaiveDate,
    pub day_data: DayDataOut,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WindowOut {
    pub id: Uuid,
    pub slots: Vec<IntervalOut>,
    pub master: MasterOut,
    pub branch: BranchOut,
}

impl Into<WindowOut> for Window {
    fn into(self) -> WindowOut {
        WindowOut {
            id: self.id,
            slots: self.slots.into_iter().map(Into::into).collect(),
            master: self.master.into(),
            branch: self.branch.into(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IntervalOut {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Into<IntervalOut> for Interval {
    fn into(self) -> IntervalOut {
        IntervalOut {
            start: self.start.and_utc(),
            end: self.end.and_utc(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum DayDataOut {
    #[serde(rename = "weekday", rename_all = "camelCase")]
    Weekday {
        branch_id: Uuid,
        working_interval: IntervalOut,
        break_intervals: Vec<IntervalOut>,
    },
    #[serde(rename = "weekend")]
    Weekend,
}

impl Into<DayDataOut> for DayData {
    fn into(self) -> DayDataOut {
        match self {
            DayData::Weekday {
                branch_id,
                working_interval,
                break_intervals,
            } => DayDataOut::Weekday {
                branch_id,
                working_interval: working_interval.into(),
                break_intervals: break_intervals
                    .into_iter()
                    .map(|interval| interval.into())
                    .collect(),
            },
            DayData::Weekend => DayDataOut::Weekend,
        }
    }
}
