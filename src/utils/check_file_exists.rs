
use std::error::Error;
use tracing::{debug, trace, info};

use crate::models::AppConfig;
use crate::utils::run_get_request::run_get_request;

pub fn check_file_exists(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    file_path: &str,  // file_path is the full path of the file in the repository
) -> Result<Option<String>, Box<dyn Error>> {
    let url_config = &*config.url_config;

    // Construct the Bitbucket API URL to check the file
    let file_url = url_config.raw_file_url(project_name, repo_name, file_path);

    info!("Checking for file {} at URL: {}", file_path, file_url);

    // Use the run_get_request helper to perform the request and get the raw response body
    let response_body = run_get_request(config, &file_url)?;

    // Check if the body contains some kind of "error" message indicating the file doesn't exist
    if response_body.contains("\"error\"") {
        info!("{} not found (file does not exist).", file_path);
        Ok(None)  // File doesn't exist
    } else if response_body.trim().is_empty() {
        info!("{} found but the file is empty.", file_path);
        Ok(None)  // Return None if the file is empty
    } else {
        info!("{} found and is not empty.", file_path);
        Ok(Some(file_url))  // Return the file URL if it exists
    }
}

