use crate::url::{UrlMode, UrlConfig};

use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use reqwest::header::{HeaderName, HeaderValue, AUTHORIZATION};
use std::env;
use crate::types::MyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BitbucketConfig {
    pub base_url: String,
}

impl UrlConfig for BitbucketConfig {

    fn raw_file_url(&self, project_name: &str, repo_name: &str, file_path: &str) -> String {
        self.file_url(UrlMode::Raw, project_name, repo_name, file_path, Some("master"))
    }

    // Common function for both raw and browse URLs, with branch parameter
    fn file_url(&self, mode: UrlMode, project_name: &str, repo_name: &str, file_path: &str, branch: Option<&str>) -> String {
        let branch = branch.unwrap_or("master"); // Default to "master" if not provided
        let mode_str = match mode {
            UrlMode::Raw => "raw",
        };

        format!(
            "{}/projects/{}/repos/{}/{}/{}?at=refs/heads/{}",
            self.base_url, project_name, repo_name, mode_str, file_path, branch
        )
    }

    fn projects_url(&self) -> String {
        format!("{}/rest/api/1.0/projects", self.base_url)
    }

    fn repos_url(&self, project_name: &str, _: &str) -> String {
        format!("{}/rest/api/1.0/projects/{}/repos", self.base_url, project_name)
    }

    // Function for getting headers
    fn get_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>, MyError> {
        dotenv().ok(); // Load environment variables

        let username = env::var("BITBUCKET_USERNAME")
            .map_err(|e| format!("Missing BITBUCKET_USERNAME: {}", e))?;
        let password = env::var("BITBUCKET_PASSWORD")
            .map_err(|e| format!("Missing BITBUCKET_PASSWORD: {}", e))?;

        let auth_value = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("{}:{}", username, password))
        );

        Ok(vec![
            (AUTHORIZATION, HeaderValue::from_str(&auth_value)?),
        ])
    }
}

