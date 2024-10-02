
use std::error::Error;

use crate::models::AppConfig;
use crate::services::get_projects::get_projects;
use crate::services::run_analysis::run_analysis;
use crate::utils::fetch_repositories::fetch_repositories;
use crate::utils::append_json_to_file::append_json_to_file; // Ensure this is imported

pub fn analyze_all_repositories(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    // Fetch projects
    let projects = get_projects(config)?;

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

        if platform == "github" {
            // For GitHub, run analysis directly without fetching repositories
            if let Some(json_data) = run_analysis(config, project_name, project_name)? {
                append_json_to_file(config, project_name, &json_data)?;  // Save the JSON
            }
        } else {
            // For Bitbucket, fetch repositories and run analysis
            let all_repos = fetch_repositories(config, project_name)?;
            for repo in all_repos {
                let repo_name = repo["name"].as_str().ok_or("Missing repo name")?;

                // Run the analysis and check if valid JSON is returned
                if let Some(json_data) = run_analysis(config, project_name, repo_name)? {
                    append_json_to_file(config, project_name, &json_data)?;  // Save the JSON
                }
            }
        }
    }
    Ok(())
}

