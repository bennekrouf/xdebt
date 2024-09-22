use std::error::Error;

use crate::models::AppConfig;
use crate::services::get_projects::get_projects;
use crate::services::run_analysis::run_analysis;
//use serde_jsn::Value;
//use tracing::{debug, error, info}; // Add tracing macros // Ensure Value is imported

use crate::utils::fetch_repositories::fetch_repositories;
pub fn analyze_all_repositories(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    let projects = get_projects(config)?;

    for project in projects {
        //info!("Project raw is : {}", project);

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

        // For GitHub, run analysis directly without fetching repositories
        if platform == "github" {
            run_analysis(config, project_name, project_name)?;
        } else {
            // For Bitbucket, fetch repositories and run analysis
            let all_repos = fetch_repositories(config, project_name)?;
            for repo in all_repos {
                let repo_name = repo["name"].as_str().ok_or("Missing repo name")?;
                run_analysis(config, project_name, repo_name)?;
            }
        }
    }
    Ok(())
}
