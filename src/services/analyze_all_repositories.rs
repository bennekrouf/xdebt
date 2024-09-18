
use std::error::Error;

use crate::utils::fetch_repositories::fetch_repositories;
use crate::services::get_projects::get_projects;
use crate::services::run_analysis::run_analysis;
use crate::create_config::AppConfig;

pub fn analyze_all_repositories(
    config: &AppConfig,
) -> Result<(), Box<dyn Error>> {
    let projects = get_projects(config)?;
    for project in projects {
        let project_name = project["key"].as_str().ok_or("Failed to get project name")?;
        let all_repos = fetch_repositories(config, project_name)?;
        for repo in all_repos {
            let repo_name = repo["name"].as_str().ok_or("Missing repo name")?;
            run_analysis(config, project_name, repo_name)?;
        }
    }
    Ok(())
}

