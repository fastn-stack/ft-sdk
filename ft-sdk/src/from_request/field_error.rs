#[derive(Debug)]
pub struct FieldError {
    pub field: &'static str,
    pub error: String,
}

impl From<FieldError> for ft_sdk::Error {
    fn from(e: FieldError) -> Self {
        let mut errors = std::collections::HashMap::new();
        errors.insert(e.field.to_string(), e.error);
        ft_sdk::Error::Form(errors)
    }
}
