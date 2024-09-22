use crate::models::UrlConfig;
// use std::env;

pub struct GithubConfig {
    pub base_url: String,
    pub user: String,
}

// impl GithubConfig {
//     pub fn new() -> Self {
//         let base_url = env::var("GITHUB_BASE_URL")
//             .unwrap_or_else(|_| "https://api.github.com".to_string()); // Adjusted to match GitHub API
//         let user = env::var("GITHUB_USER").unwrap_or_else(|_| String::new()); // Add user environment variable
//         GithubConfig { base_url, user }
//     }
// }

impl UrlConfig for GithubConfig {

    // fn base_url(&self) -> &str {
    //     &self.base_url
    // }

    fn projects_url(&self) -> String {
        format!("{}/user/repos", self.base_url)
    }

    fn repos_url(&self, _owner: &str, repo: &str) -> String {
        format!("{}/{}/{}", self.base_url, self.user, repo)
    }

    fn file_url(&self, _owner: &str, repo: &str, file_path: &str) -> String {
        format!("{}/{}/{}/contents/{}", self.base_url, self.user, repo, file_path)
    }

    fn package_json_url(&self, _owner: &str, repo: &str) -> String {
        format!("{}/{}/{}/contents/front/package.json", self.base_url, self.user, repo)
    }
}

