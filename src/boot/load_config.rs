use std::error::Error;

use crate::boot::read_yaml::read_yaml;
use crate::models::{AppConfig, ConfigFile};
use crate::url::{bitbucket::BitbucketConfig, github::GithubConfig};
use crate::utils::create_client_with_auth::create_client_with_auth;
use crate::url::UrlConfig;
use crate::boot::init_tracing::init_tracing;

pub fn load_config(config_file_path: &str) -> Result<AppConfig, Box<dyn Error>> {
    let config: ConfigFile = read_yaml(config_file_path)?;
    let _ = init_tracing(&config.trace_level.to_string())?;

    let (client, _, _) = create_client_with_auth(config.platform.clone())?;

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

    // Convert trace level to uppercase to ensure case-insensitive matching
    // let trace_level_input = config.trace.to_uppercase();
    // let trace_level: tracing::Level = match trace_level_input.as_str() {
    //     "TRACE" => tracing::Level::TRACE,
    //     "INFO" => tracing::Level::INFO,
    //     "DEBUG" => tracing::Level::DEBUG,
    //     "ERROR" => tracing::Level::ERROR,
    //     "WARN" => tracing::Level::WARN,
    //     _ => tracing::Level::ERROR, // default if not matched
    // };

    Ok(AppConfig {
        client,
        db: None, // Initialized later on
        // trace_level,
        platform: config.platform,
        output_folder: config.output_folder,
        url_config,
        force_git_pull: config.force_git_pull,
        force_maven_effective: config.force_maven_effective,
        equivalences: config.equivalences,
        enable_maven_analysis: config.enable_maven_analysis,
        enable_npm_analysis: config.enable_npm_analysis,
        enable_docker_analysis: config.enable_docker_analysis,
        enable_dotnet_analysis: config.enable_dotnet_analysis,
        enable_php_analysis: config.enable_php_analysis,
        enable_jenkins_analysis: config.enable_jenkins_analysis,
    })
}
