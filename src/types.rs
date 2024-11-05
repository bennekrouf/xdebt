use std::error::Error;
use std::fmt;
use anyhow::{anyhow, Context, Result};

pub type MyError = Box<dyn Error + Send + Sync>;

#[derive(Debug)]
pub enum CustomError {
    NotFound(String),
    AnalysisFailed(String),
    InvalidInput(String),
    ProjectError(String),
    DatabaseError(String),
    IoError(std::io::Error),
}

impl CustomError {
    pub fn project_error<T: Into<String>>(msg: T) -> MyError {
        Box::new(CustomError::ProjectError(msg.into()))
    }
    pub fn database_error<T: Into<String>>(msg: T) -> MyError {
        Box::new(CustomError::DatabaseError(msg.into()))
    }
    pub fn invalid_input<T: Into<String>>(msg: T) -> MyError {
        Box::new(CustomError::InvalidInput(msg.into()))
    }
    pub fn not_found<T: Into<String>>(msg: T) -> MyError {
        Box::new(CustomError::NotFound(msg.into()))
    }
    pub fn analysis_failed<T: Into<String>>(msg: T) -> MyError {
        Box::new(CustomError::AnalysisFailed(msg.into()))
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::AnalysisFailed(msg) => write!(f, "Analysis failed: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::ProjectError(msg) => write!(f, "Project error: {}", msg),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for CustomError {}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError::IoError(err)
    }
}
