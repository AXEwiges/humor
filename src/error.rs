use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HumorError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("YAML parsing error: {0}")]
    YamlParsing(#[from] serde_yaml::Error),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Duplicate command found: {domain}.{command}")]
    DuplicateCommand { domain: String, command: String },

    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("Invalid command structure")]
    InvalidCommandStructure,

    #[error("Command execution failed")]
    CommandExecutionFailed,
}

pub type HumorResult<T> = Result<T, HumorError>;
