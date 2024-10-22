pub mod bitbucket;
pub mod github;

pub enum UrlMode {
    Raw,
}

use reqwest::header::{HeaderName, HeaderValue};
use std::fmt::Debug;
use crate::types::MyError;

// Define a new trait for serialization
pub trait UrlConfig: Send + Sync + Debug {
    fn projects_url(&self) -> String;
    fn repos_url(&self, owner: &str, repo: &str) -> String;
    fn raw_file_url(&self, owner: &str, repo: &str, file_path: &str) -> String;
    fn file_url(&self, mode: UrlMode, owner: &str, repo: &str, file_path: &str, branch: Option<&str>) -> String;
    fn get_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>, MyError>;
}

