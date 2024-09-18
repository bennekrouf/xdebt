
use std::error::Error;
use reqwest::header::HeaderValue;
use tracing::{info, warn};

use crate::create_config::AppConfig;

pub fn check_file_exists(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    file_path: &str,  // file_path is the full path of the file in the repository
) -> Result<Option<String>, Box<dyn Error>> {
    let client = &config.client;
    let auth_header = &config.auth_header;
    let url_config = &*config.url_config;

    // Construct the Bitbucket API URL to check the file
    let file_url = url_config.file_url(project_name, repo_name, file_path);

    info!("Checking for file {} at URL: {}", file_path, file_url);

    // Make the GET request to the Bitbucket API
    let response = client
        .get(&file_url)
        .header("Authorization", HeaderValue::from_str(auth_header)?)
        .send()?;

    // Check if the response status is 200 OK
    if response.status().is_success() {
        // Read the response body
        let body = response.text()?;

        // Log the file content for debugging
        // debug!("Response content of {}:\n{}", file_path, body);

        // Check if the response contains a message indicating that the file does not exist
        if body.contains("The path") && body.contains("does not exist") {
            warn!("{} not found (path does not exist).", file_path);
            Ok(None)  // File doesn't exist
        } else if body.trim().is_empty() {
            warn!("{} found but the file is empty.", file_path);
            Ok(None)  // Return None if the file is empty
        } else {
            info!("{} found and is not empty.", file_path);
            Ok(Some(file_url))  // Return the file URL if it exists
        }
    } else if response.status() == 404 {
        warn!("{} not found (HTTP 404).", file_path);
        Ok(None)  // File doesn't exist
    } else {
        Err(format!("Failed to check {}: HTTP {}", file_path, response.status()).into())
    }
}

