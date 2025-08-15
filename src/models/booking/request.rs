use chrono::{DateTime, Utc};
use poem_openapi::{Enum, Object};
use uuid::Uuid;

use crate::models::{branch::response::BranchOut, master::response::MasterOut};

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct CreateBookingRequest {
    pub service_id: Uuid,
    pub master_id: Uuid,
    pub branch_id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub notify_methods: Vec<NotifyMethod>,
}

#[derive(Debug, Clone, Eq, PartialEq, Enum)]
pub enum NotifyMethod {
    Sms,
    Telegram,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct WindowOut {
    id: Uuid,
    slots: Vec<SlotOut>,
    master: MasterOut,
    branch: BranchOut,
}

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct SlotOut {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}
