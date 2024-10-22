
use dialoguer::Input;
use serde_json::json;

use crate::fetch_repositories::fetch_repositories;
use crate::services::get_projects::get_projects;
use crate::services::run_analysis::run_analysis;
use crate::models::AppConfig;
use crate::utils::append_json_to_file::append_json_to_file;
use crate::types::MyError;

pub fn analyze_specific_repository(
    config: &AppConfig,
    repo_name_arg: Option<&str>, // Accept repository name as an optional argument
) -> Result<(), MyError> {
    // Determine the repository name: use the argument if provided, otherwise prompt the user
    let repo_name = match repo_name_arg {
        Some(name) => name.to_string(), // Use the argument
        None => {
            // Prompt the user if no argument was provided
            Input::new()
                .with_prompt("Enter the repository name (e.g., xcad)")
                .interact()?
        }
    };

    // Fetch all projects
    let projects = get_projects(config)?;
    for project in projects {
        let project_name = project["key"].as_str().ok_or("Failed to get project name")?;

        // Initialize a vector to store analysis results for all repositories in this project
        let mut project_analysis_results = Vec::new();

        // Fetch all repositories for the project
        let all_repos = fetch_repositories(config, project_name)?;
        for repo in all_repos {
            let repo_actual_name = repo["name"].as_str().ok_or("Missing repo name")?;

            // Check if the repository matches the desired repository
            if repo_actual_name == repo_name {
                // Run the analysis and store the result
                if let Some(json_data) = run_analysis(config, &project_name, &repo_name)? {
                    project_analysis_results.push(json_data);
                }

                // Since we've found the repository, no need to continue in this project
                break;
            }
        }

        // Once all repositories for this project are analyzed, append the combined result to a file
        if !project_analysis_results.is_empty() {
            // Create a JSON array from the accumulated results
            let json_project_result = json!(project_analysis_results);
            append_json_to_file(config, &project_name, &json_project_result)?;
        }
    }

    Ok(())
}

