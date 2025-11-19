use thiserror::Error;

#[derive(Error, Debug)]
pub enum DoPlanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("State error: {0}")]
    State(String),

    #[error("Command execution error: {0}")]
    Command(String),
}

pub type Result<T> = std::result::Result<T, DoPlanError>;

