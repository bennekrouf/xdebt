
use std::error::Error;
use dialoguer::Input;

use crate::utils::fetch_repositories::fetch_repositories;
use crate::services::get_projects::get_projects;
use crate::services::run_analysis::run_analysis;
use crate::models::AppConfig;

pub fn analyze_specific_repository(
    config: &AppConfig,
    repo_name_arg: Option<&str>, // Accept repository name as an optional argument
) -> Result<(), Box<dyn Error>> {

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
        
        // Fetch all repositories for the project
        let all_repos = fetch_repositories(config, project_name)?;
        for repo in all_repos {
            let repo_actual_name = repo["name"].as_str().ok_or("Missing repo name")?;

            // Check if the repository matches the desired repository
            if repo_actual_name == repo_name {
                let _ = run_analysis(config, &project_name, &repo_name);

                // Return early as the repository has been found and analyzed
                return Ok(());
            }
        }
    }

    // If the repository is not found, return an error or a result
    Err(format!("Repository '{}' not found in any project", repo_name).into())
}

