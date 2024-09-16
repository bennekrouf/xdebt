
use std::error::Error;
use std::env;
use serde_json::Value;
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use tracing::trace;

/// Function to get the list of projects
pub fn get_projects(client: &Client, auth_header: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    // Get the base URL for the API to fetch the list of projects from the .env
    let projects_url = env::var("PROJECTS_URL")
        .map_err(|e| format!("Missing PROJECTS_URL environment variable: {}", e))?;

    // Fetch the list of projects
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


