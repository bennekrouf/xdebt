use crate::url::platform::UrlConfig;

use dotenv::dotenv;
use reqwest::header::{HeaderName, HeaderValue, AUTHORIZATION, USER_AGENT};
use std::env;
use std::error::Error;

pub struct GithubConfig {
    pub base_url: String,
    pub user: String,
}

impl UrlConfig for GithubConfig {

    fn projects_url(&self) -> String {
        format!("{}/user/repos", self.base_url)
    }

    fn repos_url(&self, _owner: &str, repo: &str) -> String {
        format!("{}/{}/{}", self.base_url, self.user, repo)
    }

    fn file_url(&self, _owner: &str, repo: &str, file_path: &str) -> String {
        format!("{}/{}/{}/contents/{}", self.base_url, self.user, repo, file_path)
    }

    fn get_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>, Box<dyn Error>> {
        dotenv().ok(); // Load environment variables

        let github_token = env::var("GITHUB_TOKEN")
            .map_err(|e| format!("Missing GITHUB_TOKEN: {}", e))?;

        let auth_value = format!("Bearer {}", github_token);
        let user_agent_value = HeaderValue::from_str("bennekrouf")?;

        Ok(vec![
            (AUTHORIZATION, HeaderValue::from_str(&auth_value)?),
            (USER_AGENT, user_agent_value),
        ])
    }
}

