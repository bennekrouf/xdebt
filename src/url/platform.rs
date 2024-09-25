
use reqwest::header::{HeaderName, HeaderValue};
use std::error::Error;

pub trait UrlConfig: Send + Sync {
    fn projects_url(&self) -> String;
    fn repos_url(&self, owner: &str, repo: &str) -> String;
    fn file_url(&self, owner: &str, repo: &str, file_path: &str) -> String;
    fn get_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>, Box<dyn Error>>;
}
