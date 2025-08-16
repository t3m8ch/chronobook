use garde::Validate;

pub fn validate_phone(value: &str, _: &()) -> garde::Result {
    if phonenumber::parse(None, value).is_ok() {
        Ok(())
    } else {
        Err(garde::Error::new("Invalid phone number"))
    }
}

pub trait ValidationExt {
    fn validate_ext(&self) -> Result<(), crate::models::error::ApiError>;
}

impl<T: Validate> ValidationExt for T
where
    <T as Validate>::Context: std::default::Default,
{
    fn validate_ext(&self) -> Result<(), crate::models::error::ApiError> {
        self.validate().map_err(|e| {
            crate::models::error::ApiError::new(
                "VALIDATION_ERROR",
                format!("Validation failed: {}", e),
            )
            .with_details(serde_json::json!({
                "errors": e.to_string()
            }))
        })
    }
}
