use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Edge connection error: {0}")]
    ConnectionError(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Unknown node type: {0}")]
    UnknownNodeType(String),

    #[error("Cycle detected in graph")]
    CycleDetected,

    #[error("Version mismatch: file version {file}, supported {supported}")]
    VersionMismatch { file: String, supported: String },
}

pub type Result<T> = std::result::Result<T, FlowError>;
