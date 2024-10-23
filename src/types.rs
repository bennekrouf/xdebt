pub type MyError = Box<dyn std::error::Error + Send + Sync>;

use std::fmt;
use std::error::Error;

// CustomError definition
#[derive(Debug)]
pub struct CustomError {
    details: String,
}

impl CustomError {
    pub fn new(msg: &str) -> MyError {
        Box::new(CustomError { details: msg.to_string() })
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CustomError {}
