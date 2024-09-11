
use reqwest::blocking::Client;
use std::error::Error;
use std::env;
use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;
use reqwest::header::AUTHORIZATION;
use dotenv::dotenv;
use tracing::trace;
use tracing_subscriber;

mod download_file;
mod run_maven_effective_pom;
mod analyze_pom_content;
mod generate_pom_analysis_json;
mod fetch_repositories;
mod append_json_to_file;
mod analyze_package_json_content;
mod append_json_to_csv;

use crate::generate_pom_analysis_json::generate_pom_analysis_json;
use crate::fetch_repositories::fetch_repositories;  // Import the new function
use crate::append_json_to_file::append_json_to_file;
use crate::append_json_to_csv::append_json_to_csv;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();  // Load environment variables from .env file

    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    // Get credentials from environment variables
    let username = env::var("BITBUCKET_USERNAME")
        .map_err(|e| format!("Missing BITBUCKET_USERNAME environment variable: {}", e))?;
    let password = env::var("BITBUCKET_PASSWORD")
        .map_err(|e| format!("Missing BITBUCKET_PASSWORD environment variable: {}", e))?;

    // Get the base URL for the API to fetch the list of projects from the .env
    let projects_url = env::var("PROJECTS_URL")
        .map_err(|e| format!("Missing PROJECTS_URL environment variable: {}", e))?;

    // Get the base URL template for repos from .env
    let repos_url_template = env::var("REPOS_URL")
        .map_err(|e| format!("Missing REPOS_URL environment variable: {}", e))?;

    // Create an HTTP client
    let client = Client::new();

    // Create an authorization header
    let auth_header = format!("Basic {}", general_purpose::STANDARD.encode(format!("{}:{}", username, password)));

    // Fetch the list of projects
    let projects_response = client
        .get(&projects_url)
        .header(AUTHORIZATION, &auth_header)
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
        .ok_or("Failed to parse projects list")?;

    // Iterate over the list of projects
    for project in projects {
        let project_name = project["key"].as_str()
            .ok_or("Failed to get project name")?;

        // Fetch repositories using the new function
        let all_repos = fetch_repositories(&client, &auth_header, &repos_url_template, project_name)?;

        // Iterate over all the repos and process each one
        for repo in &all_repos {
            let repo_name = repo["name"].as_str()
                .ok_or("Missing repo name")?;
            if repo_name.ends_with("-configuration") || repo_name.ends_with("-tests") {
                continue;
            }

            // Call the function to generate the JSON result for each repo
            match generate_pom_analysis_json(project_name, repo_name) {
                Ok(json_result) => {
                    println!("Project: {}, Repo: {}", project_name, repo_name);
                    println!("{}", serde_json::to_string_pretty(&json_result)?);

                    // Append the JSON result to the file
                    if let Err(e) = append_json_to_file(project_name, &json_result) {
                        eprintln!("Failed to append JSON to file for project '{}', repo '{}': {}", project_name, repo_name, e);
                    }

                    // Also append the JSON result to the CSV file
                    if let Err(e) = append_json_to_csv(project_name, &json_result) {
                        eprintln!("Failed to append JSON to CSV for project '{}', repo '{}': {}", project_name, repo_name, e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to generate POM analysis JSON for project '{}', repo '{}': {}", project_name, repo_name, e);
                }
            }
        }
    }

    Ok(())
}

