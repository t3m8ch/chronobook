use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MasterOut {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub patronymic: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telegram: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetMastersQuery {
    #[garde(skip)]
    pub organization_name: String,

    #[garde(skip)]
    #[serde(default)]
    pub branches: Vec<Uuid>,

    #[garde(skip)]
    #[serde(default)]
    pub services: Vec<Uuid>,
}
