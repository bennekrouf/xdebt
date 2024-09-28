use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::collections::HashMap;
// use tracing::Level;
use crate::url::UrlConfig;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub platform: String,
    pub base_url: String,
    pub user: Option<String>, // Only for GitHub
    pub force_git_pull: bool,
    pub force_maven_effective: bool,
    pub trace_level: String,
    pub output_folder: String,
    pub equivalences: HashMap<String, Vec<String>>,
    pub enable_maven_analysis: bool,
    pub enable_npm_analysis: bool,
    pub enable_docker_analysis: bool,
    pub enable_dotnet_analysis: bool,
    pub enable_php_analysis: bool,
    pub enable_jenkins_analysis: bool,
}

pub struct AppConfig {
    pub client: Client,
    pub db: Option<Db>,
    pub url_config: Box<dyn UrlConfig>,
    pub force_git_pull: bool,
    pub force_maven_effective: bool,
    // pub trace_level: Level,
    pub platform: String,
    pub output_folder: String,
    pub equivalences: HashMap<String, Vec<String>>,
    pub enable_maven_analysis: bool,
    pub enable_npm_analysis: bool,
    pub enable_docker_analysis: bool,
    pub enable_dotnet_analysis: bool,
    pub enable_php_analysis: bool,
    pub enable_jenkins_analysis: bool,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoadmapEntry {
    pub version: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    extended_end_date: Option<NaiveDate>,
    comment: Option<String>,
    source_name: Option<String>,
    source_entity: Option<String>,
    pub updated_at: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyVersion {
    pub current_version: String,
    pub dependency_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub repository_name: String,
    pub dependency_version: DependencyVersion,
    pub roadmap: Option<Roadmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KPIStatus {
    Compliant(String),           // Compliant with reason
    NonCompliant(String),        // Non-compliant with reason
    UpgradeNeeded(String),       // Upgrade needed with reason
    NoActionNeeded(String),      // No action needed with reason
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KPIResult {
    pub dependency_name: String,
    pub current_version: String,
    pub status: KPIStatus,
}
