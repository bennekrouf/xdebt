use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::blocking::Client;
use sled::Db;
use reqwest::header::HeaderValue;
use reqwest::header::HeaderName;

pub trait UrlConfig: Send + Sync {
    // fn base_url(&self) -> &str;
    fn projects_url(&self) -> String;
    fn repos_url(&self, owner: &str, repo: &str) -> String;
    fn file_url(&self, owner: &str, repo: &str, file_path: &str) -> String;
    fn package_json_url(&self, owner: &str, repo: &str) -> String;
}

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub platform: String,
    pub base_url: String,
    pub user: Option<String>,  // Only for GitHub
    pub force_git_pull: bool,
    pub force_maven_effective: bool,
    pub trace: String,
    pub output_folder: String,
    pub equivalences: HashMap<String, Vec<String>>,
}

pub struct AppConfig {
    pub client: Client,
    // pub auth_header: HeaderMap,
    pub auth_header: (HeaderName, HeaderValue),
    pub auth_user_agent: (HeaderName, HeaderValue),
    pub db: Option<Db>,
    // pub user: Option<String>,  // Only for GitHub
    pub url_config: Box<dyn UrlConfig>,
    pub force_git_pull: bool,
    pub force_maven_effective: bool,
    pub output_folder: String,
    pub equivalences: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapList {
    pub roadmap_list: Vec<Roadmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Roadmap {
    pub dependency: String,
    pub domain: Option<String>,
    pub chapter: Option<String>,
    pub entries: Vec<RoadmapEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoadmapEntry {
    pub version: String,
    start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    extended_end_date: Option<NaiveDate>,
    comment: Option<String>,
    source_name: Option<String>,
    source_entity: Option<String>,
    pub updated_at: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyVersion {
    pub version_number: String,
    pub dependency_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub repository_name: String,
    pub dependency_version: DependencyVersion,
    pub roadmap: Option<Roadmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KPIResult {
    pub dependency_name: String,
    pub version_number: String,
    pub compliance_status: String,
    pub maintenance_action: String,
}

