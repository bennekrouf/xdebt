
use reqwest::blocking::Client;
use std::error::Error;
use std::env;
use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;
use reqwest::header::AUTHORIZATION;
use dotenv::dotenv;
use tracing::{trace, error};
use tracing_subscriber;

mod download_file;
mod run_maven_effective_pom;
mod analyze_pom_content;
mod generate_pom_analysis_json;

use crate::generate_pom_analysis_json::generate_pom_analysis_json;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();  // Load environment variables from .env file

    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    // Get credentials from environment variables
    let username = env::var("BITBUCKET_USERNAME")
        .map_err(|e| format!("Missing BITBUCKET_USERNAME environment variable: {}", e))?;
    let password = env::var("BITBUCKET_PASSWORD")
        .map_err(|e| format!("Missing BITBUCKET_PASSWORD environment variable: {}", e))?;
    
    // Get the target folder from the environment
    let target_folder = env::var("TARGET_FOLDER")
        .unwrap_or_else(|_| "tmp".to_string());  // Default to "tmp" if not set

    // Get the base URL for the API to fetch the list of projects from the .env
    let projects_url = env::var("PROJECTS_URL")
        .map_err(|e| format!("Missing PROJECTS_URL environment variable: {}", e))?;

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

        // Get the list of repositories for the current project, with pagination
        let repos_url = format!(
            "https://dsigit.etat-de-vaud.ch/outils/git/rest/api/1.0/projects/{}/repos",
            project_name
        );

        // Pagination logic
        let mut start = 0;
        let limit = 50;  // Adjust limit as needed
        let mut more_pages = true;
        let mut all_repos = Vec::new();

        while more_pages {
            let paginated_repos_url = format!(
                "{}?start={}&limit={}",
                repos_url, start, limit
            );

            // Perform the HTTP GET request with basic authentication
            match client
                .get(&paginated_repos_url)
                .header(AUTHORIZATION, &auth_header)
                .send() {
                Ok(response) => {
                    if response.status().is_success() {
                        let repos_body = response.text()
                            .map_err(|e| format!("Error reading repos response body: {}", e))?;
                        trace!(
                            "Repos response body for {} (start={}): {}",
                            project_name, start, repos_body
                        );

                        let repos_json: Value = serde_json::from_str(&repos_body)
                            .map_err(|e| format!("Error parsing repos JSON: {}", e))?;
                        let repos = repos_json["values"]
                            .as_array()
                            .ok_or("Failed to parse repos list")?;

                        all_repos.extend(repos.to_vec());

                        // Check if there are more pages
                        if repos.len() < limit {
                            more_pages = false;
                        } else {
                            start += limit;
                        }
                    } else {
                        eprintln!("Failed to fetch repos, status: {}", response.status());
                        more_pages = false;
                    }
                }
                Err(err) => {
                    error!(
                        "Error fetching repos URL for project {}: {}",
                        project_name, err
                    );
                    more_pages = false;
                }
            }
        }

        // Iterate over all the repos and process each one
        for repo in &all_repos {
            let repo_name = repo["name"].as_str()
                .ok_or("Missing repo name")?;
            if repo_name.ends_with("-configuration") || repo_name.ends_with("-tests") {
                continue;
            }

            // Call the function to generate the JSON result for each repo
            match generate_pom_analysis_json(project_name, repo_name, &target_folder) {
                Ok(json_result) => {
                    println!("Project: {}, Repo: {}", project_name, repo_name);
                    println!("{}", serde_json::to_string_pretty(&json_result)?);
                }
                Err(e) => {
                    eprintln!("Failed to generate POM analysis JSON for project '{}', repo '{}': {}", project_name, repo_name, e);
                }
            }
        }
    }

    Ok(())
}

