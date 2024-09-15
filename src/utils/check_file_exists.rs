
use reqwest::blocking::Client;
use std::error::Error;
use std::env;
use reqwest::header::HeaderValue;
use tracing::{info,warn};

pub fn check_file_exists(
    client: &Client,
    auth_header: &str,
    project_name: &str,
    repo_name: &str,
    file_name: &str,
) -> Result<bool, Box<dyn Error>> {
    // Get base URL for files from .env
    let base_url = env::var("FILE_URL")
        .map_err(|e| format!("Missing FILE_URL environment variable: {}", e))?;

    // Replace placeholders in the URL
    let file_url = base_url
        .replace("{project_name}", project_name)
        .replace("{repo_name}", repo_name)
        .replace("{file_name}", file_name);

    info!("Checking for {} at URL: {}", file_name, file_url);

    // Make the request to check if the file exists
    let response = client.get(&file_url)
        .header("Authorization", HeaderValue::from_str(&auth_header)?)
        .send()?;

    // Return true if the file is found (HTTP 200), false if not (HTTP 404)
    if response.status().is_success() {
        info!("{} found.", file_name);
        Ok(true)
    } else if response.status() == 404 {
        warn!("{} not found (HTTP 404).", file_name);
        Ok(false)
    } else {
        Err(format!("Failed to check {}: HTTP {}", file_name, response.status()).into())
    }
}

