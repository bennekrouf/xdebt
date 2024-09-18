
pub trait UrlConfig {
    fn base_url(&self) -> &str;
    fn projects_url(&self) -> String;
    fn repos_url(&self, owner: &str, repo: &str) -> String;
    fn file_url(&self, owner: &str, repo: &str, file_path: &str) -> String;
    fn package_json_url(&self, owner: &str, repo: &str) -> String;
}
