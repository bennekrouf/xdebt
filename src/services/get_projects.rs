
use std::error::Error;
use serde_json::Value;
use reqwest::header::AUTHORIZATION;
use tracing::trace;

use crate::create_config::AppConfig;

pub fn get_projects(
    config: &AppConfig,
    ) -> Result<Vec<Value>, Box<dyn Error>> {
    let client = &config.client;
    let auth_header = &config.auth_header;
    let url_config = &*config.url_config; // Dereference the Box

    // Get the base URL for the API to fetch the list of projects from the .env
    let projects_url = url_config.projects_url();    // Fetch the list of projects
                                                          //
    let projects_response = client
        .get(&projects_url)
        .header(AUTHORIZATION, auth_header)
        .send()
        .map_err(|e| format!("Error fetching projects URL: {}", e))?;

    // Read the response body as text
    let projects_body = projects_response.text()
        .map_err(|e| format!("Error reading projects response body: {}", e))?;
    trace!("Projects response body: {}", projects_body);

    // Parse the JSON response
    let projects_json: Value = serde_json::from_str(&projects_body)
        .map_err(|e| format!("Error parsing projects JSON: {}", e))?;
    let projects = projects_json["values"].as_array()
        .ok_or("Failed to parse projects list")?
        .to_vec(); // Clone the array

    Ok(projects)
}


