
use std::error::Error;
use reqwest::header::HeaderValue;
use tracing::{info, warn};

use crate::create_config::AppConfig;

pub fn check_file_exists(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    file_name: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let client = &config.client;
    let auth_header = &config.auth_header;
    let url_config = &*config.url_config; // Dereference the Box

    let file_url = url_config.file_url(project_name, repo_name, file_name);
    // Get base URL for files from .env
    // let base_url = env::var("FILE_URL")
        // .map_err(|e| format!("Missing FILE_URL environment variable: {}", e))?;

    // Replace placeholders in the URL
    // let file_url = base_url
    //     .replace("{project_name}", project_name)
    //     .replace("{repo_name}", repo_name)
    //     .replace("{file_name}", file_name);

    info!("Checking for {} at URL: {}", file_name, file_url);

    // Make the request to check if the file exists
    let response = client.get(&file_url)
        .header("Authorization", HeaderValue::from_str(auth_header)?)
        .send()?;

    // Return the URL if the file is found (HTTP 200), or None if not (HTTP 404)
    if response.status().is_success() {
        info!("{} found.", file_name);
        Ok(Some(file_url))
    } else if response.status() == 404 {
        warn!("{} not found (HTTP 404).", file_name);
        Ok(None)
    } else {
        Err(format!("Failed to check {}: HTTP {}", file_name, response.status()).into())
    }
}

