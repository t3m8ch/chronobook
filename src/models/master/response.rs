use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct MasterOut {
    id: Uuid,
    first_name: String,
    last_name: String,
    patronymic: Option<String>,
    contact_phone: Option<String>,
    contact_email: Option<String>,
    contact_telegram: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetMastersQuery {
    pub organization_id: Uuid,
    #[serde(default)]
    pub branches: Vec<Uuid>,
}
