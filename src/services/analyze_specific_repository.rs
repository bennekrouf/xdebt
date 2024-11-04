use dialoguer::Input;
use std::error::Error;

use crate::models::AppConfig;
use crate::services::get_projects::get_projects;
use crate::services::run_analysis::run_analysis;
use crate::utils::fetch_repositories::fetch_repositories;

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

    // Handle platform-specific logic with match
    match config.platform.as_str() {
        "bitbucket" => {
            // Bitbucket: Loop through projects, then repos
            let projects = get_projects(config)?;
            for project in projects {
                let project_name = project["key"]
                    .as_str()
                    .ok_or("2 - Failed to get project name")?;
                println!("Project fetched (Bitbucket): {}", project_name);

                let all_repos = fetch_repositories(config, project_name)?;
                for repo in all_repos {
                    let repo_actual_name = repo["name"].as_str().ok_or("Missing repo name")?;
                    if repo_actual_name == repo_name {
                        run_analysis(config, project_name, &repo_name)?;
                    }
                }
            }
        }
        "github" => {
            // GitHub: Skip projects, directly fetch all repos for the user
            let user_repos = fetch_repositories(config, "bennekrouf")?;
            for repo in user_repos {
                let repo_actual_name = repo["name"].as_str().ok_or("Missing repo name")?;
                if repo_actual_name == repo_name {
                    println!("Repository fetched (GitHub): {}", repo_actual_name);
                    run_analysis(config, "user", &repo_name)?; // No project name needed for GitHub
                }
            }
        }
        // Future platform extensions can easily be added here
        _ => {
            return Err("Unsupported platform".into());
        }
    }

    Ok(())
}
