use poem_openapi::Object;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct CreateServiceRequest {
    pub display_name: String,
    pub description: String,
    pub duration_minutes: Option<i32>,
    pub price: String,
    pub master_id: Option<Uuid>,
}
