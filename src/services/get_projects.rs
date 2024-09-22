use serde_json::Value;
use std::error::Error;
use tracing::{debug, error, info, trace}; // Add tracing macros

use crate::models::AppConfig;
use crate::utils::run_get_query::run_get_query; // Import run_get_query

pub fn get_projects(config: &AppConfig) -> Result<Vec<Value>, Box<dyn Error>> {
    let url_config = &*config.url_config; // Dereference the Box

    // Get the base URL for the API to fetch the list of projects
    let projects_url = url_config.projects_url(); // Fetch the list of projects
    info!("Fetching projects from URL: {}", projects_url);

    // Call run_get_query to perform the GET request
    let response_json = run_get_query(config, &projects_url)?;

    // Check if the response contains an error
    if response_json.get("error").is_some() {
        return Err(format!(
            "Failed to fetch projects: Error in response from '{}'",
            projects_url
        )
        .into());
    }

    // Determine the platform and parse projects accordingly
    let platform = &config.platform; // Assuming you have a field `platform` in your config
    let projects = match platform.as_str() {
        "bitbucket" => {
            // For Bitbucket, extract projects from the `values` key
            response_json["values"]
                .as_array()
                .ok_or("Failed to parse Bitbucket projects list")?
                .to_vec()
        }
        "github" => {
            // For GitHub, extract directly from the response array
            response_json
                .as_array()
                .ok_or("Failed to parse GitHub projects list")?
                .to_vec()
        }
        _ => {
            return Err("Unsupported platform".into());
        }
    };

    Ok(projects)
}
