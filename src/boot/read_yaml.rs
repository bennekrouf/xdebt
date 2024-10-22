
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::fmt;

use crate::models::ConfigFile;

#[derive(Debug)]
pub enum ConfigError {
    FileOpenError(String),
    FileReadError(String),
    YamlParseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileOpenError(msg) => write!(f, "Failed to open file: {}", msg),
            ConfigError::FileReadError(msg) => write!(f, "Failed to read file: {}", msg),
            ConfigError::YamlParseError(msg) => write!(f, "Failed to parse YAML: {}", msg),
        }
    }
}

impl Error for ConfigError {}

use crate::types::MyError;
pub fn read_yaml(file_path: &str) -> Result<ConfigFile, MyError> {
    let mut file = File::open(&file_path)
        .map_err(|e| ConfigError::FileOpenError(format!("{}: {}", file_path, e)))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| ConfigError::FileReadError(format!("{}: {}", file_path, e)))?;

    let config: ConfigFile = serde_yaml::from_str(&contents)
        .map_err(|e| ConfigError::YamlParseError(format!("{}: {}", file_path, e)))?;

    Ok(config)
}

