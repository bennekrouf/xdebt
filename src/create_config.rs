use std::error::Error;
use std::env;
use reqwest::blocking::Client;
use sled::Db;

use crate::utils::create_client_with_auth::create_client_with_auth;
use crate::url::bitbucket::BitbucketConfig;
use crate::url::github::GithubConfig;
use crate::url::models::UrlConfig;

pub struct AppConfig {
    pub client: Client,
    pub auth_header: String,
    pub db: Db,
    pub url_config: Box<dyn UrlConfig>,
    pub force_git_pull: bool,
}

pub fn create_config() -> Result<AppConfig, Box<dyn Error>> {
    let (client, auth_header) = create_client_with_auth()?;
    let db = sled::open("roadmap_db").expect("Failed to open DB");
 
    // Read environment variables
    let platform_name = env::var("PLATFORM")
        .map_err(|e| format!("Missing PLATFORM environment variable: {}", e))?;
    let base_url = env::var("BASE_URL")
        .map_err(|e| format!("Missing BASE_URL environment variable: {}", e))?;
    let user = env::var("USER").unwrap_or_default();  // Only for GitHub

    // Match platform and construct corresponding URL config
    let url_config: Box<dyn UrlConfig> = match platform_name.as_str() {
        "bitbucket" => Box::new(BitbucketConfig { base_url }),
        "github" => Box::new(GithubConfig { base_url, user }),
        _ => return Err("Unsupported platform".into()),
    };

    // Get FORCE_GIT_PULL from .env
    let force_git_pull = env::var("FORCE_GIT_PULL")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false); // Default to `false` if parsing fails

    Ok(AppConfig {
        client,
        auth_header,
        db,
        url_config,
        force_git_pull,
    })
}
