use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum FlowError {
    #[error("IO error: {0}")]
    Io(String),

    #[error("JSON parse error: {0}")]
    Json(String),

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

    #[error("Cycle detected in graph: {0:?}")]
    CycleDetected(Vec<String>),

    #[error("Version mismatch: file version {file}, supported {supported}")]
    VersionMismatch { file: String, supported: String },

    /// 非阻塞警告：不影响代码生成，但在编辑器状态栏提示。
    #[error("{0}")]
    Warning(String),
}

impl FlowError {
    /// 返回受此错误影响的节点 ID 列表。
    pub fn affected_node_ids(&self) -> Vec<String> {
        match self {
            Self::CycleDetected(ids) => ids.clone(),
            Self::NodeNotFound(id) => vec![id.clone()],
            _ => Vec::new(),
        }
    }

    /// 是否为阻塞错误（vs. 非阻塞警告）。
    pub fn is_blocking(&self) -> bool {
        !matches!(self, Self::Warning(_))
    }

    /// 是否为警告。
    pub fn is_warning(&self) -> bool {
        matches!(self, Self::Warning(_))
    }
}

impl From<std::io::Error> for FlowError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

impl From<serde_json::Error> for FlowError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e.to_string())
    }
}

impl From<uuid::Error> for FlowError {
    fn from(_: uuid::Error) -> Self {
        Self::Validation("UUID parse error".to_string())
    }
}

pub type Result<T> = std::result::Result<T, FlowError>;
