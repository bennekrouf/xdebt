
use crate::url::{UrlMode, UrlConfig};
use dotenv::dotenv;
use reqwest::header::{HeaderName, HeaderValue, AUTHORIZATION, USER_AGENT};
use std::env;
use std::error::Error;

pub struct GithubConfig {
    pub base_url: String,
    pub user: String,
}

impl UrlConfig for GithubConfig {
    // URL for user's repositories
    fn projects_url(&self) -> String {
        format!("{}/user/repos", self.base_url)
    }

    // URL for accessing a specific repository
    fn repos_url(&self, _owner: &str, repo: &str) -> String {
        format!("{}/{}/{}", self.base_url, self.user, repo)
    }

    // Unified URL for accessing file content (GitHub uses 'contents' endpoint for raw access)
    fn raw_file_url(&self, _owner: &str, repo: &str, file_path: &str) -> String {
        self.file_url(UrlMode::Raw, _owner, repo, file_path, None)
    }

    // For GitHub, the `file_url` and `raw_file_url` can be the same (accessing contents)
    fn file_url(&self, _mode: UrlMode, _owner: &str, repo: &str, file_path: &str, _branch: Option<&str>) -> String {
        format!("{}/{}/{}/contents/{}", self.base_url, self.user, repo, file_path)
    }

    // Method to get necessary headers, including GitHub token and user agent
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

// Enum to distinguish URL modes (if necessary)
// pub enum UrlMode {
//     Raw,
//     Browse,
// }

