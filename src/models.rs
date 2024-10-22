use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize, Serializer, ser::SerializeStruct};
use sled::Db;
use std::collections::HashMap;

use crate::url::bitbucket::BitbucketConfig;
use crate::url::UrlConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigFile {
    pub platform: String,
    pub base_url: String,
    pub user: Option<String>, // Only for GitHub
    pub force_git_pull: bool,
    pub force_maven_effective: bool,
    pub force_sled_db_sourcing: bool,
    pub trace_level: String,
    pub output_folder: String,
    pub roadmap_folder: String,
    pub sources_priorities: Option<Vec<String>>,
    pub equivalences: HashMap<String, Vec<String>>,
    pub enable_maven_analysis: bool,
    pub enable_npm_analysis: bool,
    pub enable_docker_analysis: bool,
    pub enable_dotnet_analysis: bool,
    pub enable_php_analysis: bool,
    pub enable_jenkins_analysis: bool,
}

// Custom deserialization function for `url_config`
fn default_url_config() -> Box<dyn UrlConfig> {
    Box::new(BitbucketConfig {
        base_url: "https://bitbucket.org".to_string(),
    })
}

use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub client: Client,
    pub db: Option<Db>,
    pub url_config: Arc<dyn UrlConfig>,
    pub force_git_pull: bool,
    pub force_maven_effective: bool,
    pub force_sled_db_sourcing: bool,
    pub platform: String,
    pub output_folder: String,
    pub roadmap_folder: String,
    pub sources_priorities: Option<Vec<String>>,
    pub equivalences: HashMap<String, Vec<String>>,
    pub enable_maven_analysis: bool,
    pub enable_npm_analysis: bool,
    pub enable_docker_analysis: bool,
    pub enable_dotnet_analysis: bool,
    pub enable_php_analysis: bool,
    pub enable_jenkins_analysis: bool,
}

// Manually implement Default for AppConfig
impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            client: Client::new(),           // Initialize with appropriate default
            db: None,                        // Default to None
            url_config: default_url_config().into(), // Use the custom default URL config
            force_git_pull: false,
            force_maven_effective: false,
            force_sled_db_sourcing: false,
            platform: "bitbucket".to_string(), // Default platform
            output_folder: "tmp".to_string(),      // Default output folder
            roadmap_folder: "roadmap".to_string(),    // Default roadmap folder
            sources_priorities: None,
            equivalences: HashMap::new(),
            enable_maven_analysis: false,
            enable_npm_analysis: false,
            enable_docker_analysis: false,
            enable_dotnet_analysis: false,
            enable_php_analysis: false,
            enable_jenkins_analysis: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Roadmaps {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyVersion {
    pub cycle: String,
    pub product: String,
}

#[derive(Debug, Deserialize)]
pub struct Analysis {
    pub repository_name: String,
    pub dependency_version: DependencyVersion,
    pub roadmap: Option<Roadmap>,
}

impl Serialize for Analysis {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Define the number of fields to serialize
        let mut state = serializer.serialize_struct("Analysis", 3)?;

        // Serialize the fields in the custom order
        state.serialize_field("repository_name", &self.repository_name)?;
        state.serialize_field("dependency_version", &self.dependency_version)?;
        state.serialize_field("roadmap", &self.roadmap)?;

        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KPIStatus {
    Compliant,
    NonCompliant,
    UpToDate,
    Outdated,
    NoActionNeeded,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KPIResult {
    pub product: String,
    pub cycle: String,
    pub status: KPIStatus,
    pub reason: String,
    pub source: Option<String>,
    pub validity: Option<String>,
}


