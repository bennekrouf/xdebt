use crate::url::models::UrlConfig;
// use std::env;

pub struct BitbucketConfig {
    pub base_url: String,
}

// impl BitbucketConfig {
//     pub fn new() -> Self {
//         let base_url = env::var("BITBUCKET_BASE_URL")
//             .unwrap_or_else(|_| "https://dsigit.etat-de-vaud.ch/outils/git".to_string());
//         BitbucketConfig { base_url }
//     }
// }

impl UrlConfig for BitbucketConfig {
    // fn base_url(&self) -> &str {
    //     &self.base_url
    // }

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

    fn package_json_url(&self, project_name: &str, repo_name: &str) -> String {
        format!("{}/rest/api/latest/projects/{}/repos/{}/browse/front/package.json?at=refs/heads/master", self.base_url, project_name, repo_name)
    }
}

