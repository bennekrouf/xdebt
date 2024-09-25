use crate::url::platform::UrlConfig;

use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use reqwest::header::{HeaderName, HeaderValue, AUTHORIZATION};
use std::env;
use std::error::Error;

pub struct BitbucketConfig {
    pub base_url: String,
}

impl UrlConfig for BitbucketConfig {

    fn projects_url(&self) -> String {
        format!("{}/rest/api/1.0/projects", self.base_url)
    }

    fn repos_url(&self, project_name: &str, _: &str) -> String {
        format!("{}/rest/api/1.0/projects/{}/repos", self.base_url, project_name)
    }

    fn file_url(&self, project_name: &str, repo_name: &str, file_path: &str) -> String {
        // format!("{}/projects/{}/repos/{}/browse/{}", self.base_url, project_name, repo_name, file_path)
        format!("{}/projects/{}/repos/{}/raw/{}?at=refs/heads/master", self.base_url, project_name, repo_name, file_path)
    }

    fn get_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>, Box<dyn Error>> {
        dotenv().ok(); // Load environment variables

        let username = env::var("BITBUCKET_USERNAME")
            .map_err(|e| format!("Missing BITBUCKET_USERNAME: {}", e))?;
        let password = env::var("BITBUCKET_PASSWORD")
            .map_err(|e| format!("Missing BITBUCKET_PASSWORD: {}", e))?;

        let auth_value = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("{}:{}", username, password))
        );
        // let user_agent_value = HeaderValue::from_str("bennekrouf")?;

        Ok(vec![
            (AUTHORIZATION, HeaderValue::from_str(&auth_value)?),
            // (USER_AGENT, user_agent_value),
        ])
    }
}

