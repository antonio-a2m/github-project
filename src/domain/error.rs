use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation Error: {0}")]
    ValidationError(String),
    // Example: #[error("Entity not found: {entity_type} with id {id}")]
    // NotFound { entity_type: String, id: String },
}
