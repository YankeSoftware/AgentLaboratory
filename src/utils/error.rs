use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Test fixture error: {0}")]
    Fixture(String),

    #[error("File operation failed on {path}: {message}")]
    FileOp {
        path: PathBuf,
        message: String,
    },

    #[error("API error: {0}")]
    Api(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Model error: {0}")]
    Model(String),

    #[error("PDF processing error: {0}")]
    PdfProcessing(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type AgentResult<T> = Result<T, AgentError>;