
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use reqwest::blocking::Client;
use sled::Db;
use std::fmt;

use crate::utils::create_client_with_auth::create_client_with_auth;
use crate::url::bitbucket::BitbucketConfig;
use crate::url::github::GithubConfig;
use crate::url::models::UrlConfig;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    platform: String,
    base_url: String,
    user: Option<String>,  // Only for GitHub
    force_git_pull: bool,
    force_maven_effective: bool,
    trace: String,
    equivalences: HashMap<String, Vec<String>>,
}

pub struct AppConfig {
    pub client: Client,
    pub auth_header: String,
    pub db: Db,
    pub url_config: Box<dyn UrlConfig>,
    pub force_git_pull: bool,
    pub force_maven_effective: bool,
    pub equivalences: HashMap<String, Vec<String>>,
}

pub fn create_config() -> Result<AppConfig, Box<dyn Error>> {
    // Read the YAML configuration file
    let config_file_path = "configuration.yml";
    let config: ConfigFile = read_yaml(config_file_path)?;

    let trace_level: tracing::Level = match config.trace.as_str() {
        "INFO" => tracing::Level::INFO,
        "DEBUG" => tracing::Level::DEBUG,
        "ERROR" => tracing::Level::ERROR,
        "WARN" => tracing::Level::WARN,
        _ => tracing::Level::INFO, // default to INFO if not matched
    };

    tracing_subscriber::fmt().with_max_level(trace_level).init();
    let (client, auth_header) = create_client_with_auth()?;
    let db = sled::open("roadmap_db").expect("Failed to open DB");

    // Match platform and construct the corresponding URL config
    let url_config: Box<dyn UrlConfig> = match config.platform.as_str() {
        "bitbucket" => Box::new(BitbucketConfig { base_url: config.base_url.clone() }),
        "github" => Box::new(GithubConfig { base_url: config.base_url.clone(), user: config.user.clone().unwrap_or_default() }),
        _ => return Err("Unsupported platform".into()),
    };

    Ok(AppConfig {
        client,
        auth_header,
        db,
        url_config,
        force_git_pull: config.force_git_pull,
        force_maven_effective: config.force_maven_effective,
        equivalences: config.equivalences,
    })
}

#[derive(Debug)]
pub enum ConfigError {
    FileOpenError(String),
    FileReadError(String),
    YamlParseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileOpenError(msg) => write!(f, "Failed to open file: {}", msg),
            ConfigError::FileReadError(msg) => write!(f, "Failed to read file: {}", msg),
            ConfigError::YamlParseError(msg) => write!(f, "Failed to parse YAML: {}", msg),
        }
    }
}

impl Error for ConfigError {}

fn read_yaml(file_path: &str) -> Result<ConfigFile, Box<dyn Error>> {
    let mut file = File::open(file_path)
        .map_err(|e| ConfigError::FileOpenError(format!("{}: {}", file_path, e)))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| ConfigError::FileReadError(format!("{}: {}", file_path, e)))?;

    let config: ConfigFile = serde_yaml::from_str(&contents)
        .map_err(|e| ConfigError::YamlParseError(format!("{}: {}", file_path, e)))?;

    Ok(config)
}




