use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::{branch::response::BranchOut, master::response::MasterOut};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct CreateBookingRequest {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub service_id: Uuid,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440001")]
    pub master_id: Uuid,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440002")]
    pub branch_id: Uuid,

    #[schema(example = "2024-01-01T10:00:00Z")]
    pub start: DateTime<Utc>,

    #[schema(example = "2024-01-01T11:00:00Z")]
    pub end: DateTime<Utc>,

    pub notify_methods: Vec<NotifyMethod>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum NotifyMethod {
    Sms,
    Telegram,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct WindowOut {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    pub slots: Vec<SlotOut>,
    pub master: MasterOut,
    pub branch: BranchOut,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct SlotOut {
    #[schema(example = "2024-01-01T10:00:00Z")]
    pub start_time: DateTime<Utc>,

    #[schema(example = "2024-01-01T11:00:00Z")]
    pub end_time: DateTime<Utc>,
}
