use garde::Validate;
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, IntoParams)]
#[into_params(parameter_in = Query)]
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
