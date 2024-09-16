
use std::error::Error;
use dialoguer::Input;

use crate::utils::fetch_repositories::fetch_repositories;
use crate::services::get_projects::get_projects;
use crate::services::run_analysis::run_analysis;

pub fn analyze_specific_repository(
    db: &sled::Db,
    client: &reqwest::blocking::Client,
    auth_header: &str,
    repos_url_template: &str
) -> Result<(), Box<dyn Error>> {
    let repo_name: String = Input::new()
        .with_prompt("Enter the repository name (e.g., xcad):")
        .interact()?;

    let projects = get_projects(client, auth_header)?;
    for project in projects {
        let project_name = project["key"].as_str().ok_or("Failed to get project name")?;
        let all_repos = fetch_repositories(client, auth_header, repos_url_template, project_name)?;
        for repo in all_repos {
            let repo_actual_name = repo["name"].as_str().ok_or("Missing repo name")?;
            if repo_actual_name == repo_name {
                let _ = run_analysis(db, client, auth_header, &project_name, &repo_name);
            }
        }
    }
    Ok(())
}

