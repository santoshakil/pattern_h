#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{entity_type} not found: {id}")]
    NotFound { entity_type: String, id: String },
    #[error("validation: {0}")]
    Validation(String),
    #[error("invalid state: {from} -> {to}")]
    InvalidState { from: String, to: String },
    #[error("permission denied: {action} on {resource}")]
    PermissionDenied { action: String, resource: String },
    #[error("duplicate {entity_type}.{field} = {value}")]
    Duplicate {
        entity_type: String,
        field: String,
        value: String,
    },
    #[error("business rule: {0}")]
    BusinessRule(String),
}
