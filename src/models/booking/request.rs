use chrono::NaiveDateTime;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookingRequest {
    #[garde(skip)]
    pub organization_name: String,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    #[garde(skip)]
    pub service_id: Uuid,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440001")]
    #[garde(skip)]
    pub master_id: Uuid,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440002")]
    #[garde(skip)]
    pub branch_id: Uuid,

    #[schema(example = "2024-01-01T10:00:00")]
    #[garde(skip)]
    pub start: NaiveDateTime,

    #[schema(example = "2024-01-01T11:00:00")]
    #[garde(skip)]
    pub end: NaiveDateTime,

    #[garde(skip)]
    pub notify_methods: Vec<NotifyMethod>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum NotifyMethod {
    Sms,
    Telegram,
}
