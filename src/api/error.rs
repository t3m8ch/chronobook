use poem_openapi::Object;

#[derive(Debug, Clone, Eq, PartialEq, Object)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}
