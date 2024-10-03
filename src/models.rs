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
    pub product: String,
    pub domain: Option<String>,
    pub chapter: Option<String>,
    pub entries: Vec<RoadmapEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoadmapEntry {
    pub cycle: String,
    pub release_date: Option<NaiveDate>,
    pub eol: Option<NaiveDate>,
    pub extended_end_date: Option<NaiveDate>,
    pub comment: Option<String>,
    pub source_name: Option<String>,
    pub source_entity: Option<String>,
    pub updated_at: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyVersion {
    pub cycle: String,
    pub product: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub repository_name: String,
    pub dependency_version: DependencyVersion,
    pub roadmap: Option<Roadmap>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KPIStatus {
    Compliant,           // Compliant status
    NonCompliant,        // Non-compliant status
    Outdated,       // Upgrade needed status
    NoActionNeeded,      // No action needed status
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KPIResult {
    pub product: String,
    pub cycle: String,
    pub status: KPIStatus,
    pub reason: String,  // New field for the reason
}
