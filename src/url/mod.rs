pub mod bitbucket;
pub mod github;

pub enum UrlMode {
    Raw,
}

use reqwest::header::{HeaderName, HeaderValue};
use std::error::Error;

pub trait UrlConfig: Send + Sync {
    fn projects_url(&self) -> String;
    fn repos_url(&self, owner: &str, repo: &str) -> String;
    fn raw_file_url(&self, owner: &str, repo: &str, file_path: &str) -> String;
    fn file_url(&self, url_mode: UrlMode, owner: &str, repo: &str, file_path: &str, branch: Option<&str>) -> String;
    fn get_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>, Box<dyn Error>>;
}
