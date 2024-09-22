use std::error::Error;

use crate::boot::read_yaml::read_yaml;
use crate::models::{AppConfig, ConfigFile, UrlConfig};
use crate::url::{bitbucket::BitbucketConfig, github::GithubConfig};
use crate::utils::create_client_with_auth::create_client_with_auth;

pub fn load_config(config_file_path: &str) -> Result<AppConfig, Box<dyn Error>> {
    let config: ConfigFile = read_yaml(config_file_path)?;

    let (client, auth_header, auth_user_agent) = create_client_with_auth(config.platform.clone())?;
    // let db = sled::open("roadmap_db")?;

    // Match platform and construct the corresponding URL config
    let url_config: Box<dyn UrlConfig> = match config.platform.as_str() {
        "bitbucket" => Box::new(BitbucketConfig {
            base_url: config.base_url.clone(),
        }),
        "github" => Box::new(GithubConfig {
            base_url: config.base_url.clone(),
            user: config.user.clone().unwrap_or_default(),
        }),
        _ => return Err("Unsupported platform".into()),
    };

    let trace_level: tracing::Level = match config.trace.as_str() {
        "INFO" => tracing::Level::INFO,
        "DEBUG" => tracing::Level::DEBUG,
        "ERROR" => tracing::Level::ERROR,
        "WARN" => tracing::Level::WARN,
        _ => tracing::Level::INFO, // default to INFO if not matched
    };

    Ok(AppConfig {
        client,
        auth_header,
        auth_user_agent,
        db: None,
        trace_level,
        output_folder: config.output_folder,
        url_config,
        force_git_pull: config.force_git_pull,
        force_maven_effective: config.force_maven_effective,
        equivalences: config.equivalences,
    })
}
