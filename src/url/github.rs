use crate::url::{UrlConfig, UrlMode};
use dotenv::dotenv;
use reqwest::header::{HeaderName, HeaderValue, AUTHORIZATION, USER_AGENT};
use std::env;
use std::error::Error;

pub struct GithubConfig {
    pub base_url: String, // GitHub API base URL (https://api.github.com)
    pub user: String,     // GitHub username or organization
}

impl UrlConfig for GithubConfig {
    // URL for listing repositories for a given user or organization
    fn projects_url(&self) -> String {
        // API URL to get a list of repositories for the authenticated user or a specific organization
        format!("{}/users/{}/repos", self.base_url, self.user)
    }

    // URL for accessing a specific repository (using API)
    fn repos_url(&self, owner: &str, repo: &str) -> String {
        format!("{}/users/{}/repos", self.base_url, self.user)
        //format!("{}/repos/{}/{}", self.base_url, owner, repo)
    }

    // URL for accessing the raw content of a file in the GitHub API
    fn raw_file_url(&self, owner: &str, repo: &str, file_path: &str) -> String {
        self.file_url(UrlMode::Raw, owner, repo, file_path, None)
    }

    // Generate URL for a file in the GitHub UI (blob format)
    fn file_url(
        &self,
        _mode: UrlMode,
        owner: &str,
        repo: &str,
        file_path: &str,
        branch: Option<&str>,
    ) -> String {
        let branch = branch.unwrap_or("master"); // Default to 'master' if no branch is specified
        format!(
            "https://github.com/{}/{}/blob/{}/{}",
            owner, repo, branch, file_path
        )
    }

    // Method to get necessary headers, including GitHub token and user agent
    fn get_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>, Box<dyn Error>> {
        dotenv().ok(); // Load environment variables

        let github_token =
            env::var("GITHUB_TOKEN").map_err(|e| format!("Missing GITHUB_TOKEN: {}", e))?;

        let auth_value = format!("Bearer {}", github_token);
        let user_agent_value = HeaderValue::from_str("bennekrouf")?;

        Ok(vec![
            (AUTHORIZATION, HeaderValue::from_str(&auth_value)?),
            (USER_AGENT, user_agent_value),
        ])
    }
}
