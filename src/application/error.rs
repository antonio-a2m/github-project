use thiserror::Error;
use crate::domain::error::DomainError;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Repository operation failed: {0}")]
    RepositoryError(#[from] anyhow::Error), // Catch-all for repository errors

    #[error("Provider operation failed: {source_error}")]
    ProviderError { source_error: anyhow::Error }, // Specific for provider errors

    #[error("Domain logic error: {0}")]
    DomainError(#[from] DomainError),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Data processing error: {0}")]
    ProcessingError(String),

    #[error("Operation failed unexpectedly: {0}")]
    UnexpectedError(String),
}
