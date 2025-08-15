use derive_more::Display;
use poem_openapi::Validator;

#[derive(Debug, Clone, Copy, Display)]
#[display("invalid phone number")]
pub struct PhoneValidator;

impl Validator<String> for PhoneValidator {
    fn check(&self, value: &String) -> bool {
        phonenumber::parse(None, value).is_ok()
    }
}
