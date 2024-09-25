
use std::error::Error;
use tracing::{debug, trace};

use crate::models::AppConfig;
use crate::utils::run_json_get_query::run_json_get_query;

pub fn check_file_exists(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    file_path: &str,  // file_path is the full path of the file in the repository
) -> Result<Option<String>, Box<dyn Error>> {
    let url_config = &*config.url_config;

    // Construct the Bitbucket API URL to check the file
    let file_url = url_config.file_url(project_name, repo_name, file_path);

    trace!("Checking for file {} at URL: {}", file_path, file_url);

    // Use the run_json_get_query helper to perform the request
    let response_json = run_json_get_query(config, &file_url)?;

    // Check for existence based on response content
    if response_json.get("error").is_some() {
        debug!("{} not found (file does not exist).", file_path);
        Ok(None)  // File doesn't exist
    } else if response_json.get("content").is_some() && response_json["content"].as_str().unwrap_or("").trim().is_empty() {
        debug!("{} found but the file is empty.", file_path);
        Ok(None)  // Return None if the file is empty
    } else {
        debug!("{} found and is not empty.", file_path);
        Ok(Some(file_url))  // Return the file URL if it exists
    }
}

