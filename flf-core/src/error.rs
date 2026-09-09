//! Error types for the FLF framework

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No valid structures could be generated")]
    NoValidStructures,

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Evolution failed: {0}")]
    EvolutionFailed(String),

    #[error("Problem representation failed: {0}")]
    RepresentationFailed(String),

    #[error("Field error: {0}")]
    FieldError(String),

    #[error("Structure mutation failed: {0}")]
    MutationFailed(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
