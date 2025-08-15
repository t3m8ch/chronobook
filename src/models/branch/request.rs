use poem_openapi::Object;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct CreateBranchRequest {
    name: String,
    description: String,
    timezone: String,
    street: String,
    house_number: String,
    apartment_number: String,
    city: String,
    region: String,
    country: String,
    address_info: Option<String>,
    organization_id: Uuid,
}
