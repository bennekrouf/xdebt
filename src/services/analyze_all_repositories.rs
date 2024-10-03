
use std::collections::HashMap;
use std::error::Error;
use serde_json::{Value, json};

use crate::models::AppConfig;
use crate::services::get_projects::get_projects;
use crate::services::run_analysis::run_analysis;
use crate::utils::fetch_repositories::fetch_repositories;
use crate::utils::append_json_to_file::append_json_to_file;

pub fn analyze_all_repositories(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    // Fetch projects
    let projects = get_projects(config)?;

    // Initialize a HashMap to accumulate analysis results for all repositories grouped by project
    let mut all_analysis_results: HashMap<String, Vec<Value>> = HashMap::new();

    for project in projects {
        // Determine the platform
        let platform = &config.platform; // Assuming platform is stored in the config

        // Extract project name based on platform
        let project_name = match platform.as_str() {
            "github" => project["full_name"]
                .as_str()
                .ok_or("Failed to get project name")?, // Use full_name for GitHub
            "bitbucket" => project["key"]
                .as_str()
                .ok_or("Failed to get project name")?, // Use key for Bitbucket
            _ => return Err("Unsupported platform".into()),
        };

        // Initialize a vector to accumulate analysis results for the current project
        let mut project_analysis_results = Vec::new();

        if platform == "github" {
            // For GitHub, run analysis directly without fetching repositories
            if let Some(json_data) = run_analysis(config, project_name, project_name)? {
                // Accumulate the analysis result for the current project
                project_analysis_results.push(json_data.clone());
            }
        } else {
            // For Bitbucket, fetch repositories and run analysis
            let all_repos = fetch_repositories(config, project_name)?;
            for repo in all_repos {
                let repo_name = repo["name"].as_str().ok_or("Missing repo name")?;

                // Run the analysis and check if valid JSON is returned
                if let Some(json_data) = run_analysis(config, project_name, repo_name)? {
                    // Accumulate the analysis result for the current project
                    project_analysis_results.push(json_data.clone());
                }
            }
        }

        // After processing all repositories of the current project, append the results to a project-specific JSON file
        if !project_analysis_results.is_empty() {
            let json_project_result = json!(project_analysis_results);
            append_json_to_file(config, project_name, &json_project_result)?;  // Save per-project JSON

            // Add the project and its repositories to the `all_analysis_results`
            all_analysis_results.insert(project_name.to_string(), project_analysis_results);
        }
    }

    // After processing all projects and repositories, append the accumulated results to the `all_projects.json` file
    if !all_analysis_results.is_empty() {
        let json_all_projects_result = json!(all_analysis_results);
        append_json_to_file(config, "all_projects", &json_all_projects_result)?;  // Save all projects JSON with nested structure
    }

    Ok(())
}

