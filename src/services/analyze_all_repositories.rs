
use std::error::Error;

use crate::utils::fetch_repositories::fetch_repositories;
use crate::utils::get_projects::get_projects;

use crate::services::run_analysis::run_analysis;
pub fn analyze_all_repositories(
    client: &reqwest::blocking::Client,
    auth_header: &str,
    repos_url_template: &str
) -> Result<(), Box<dyn Error>> {
    let projects = get_projects(client, auth_header)?;
    for project in projects {
        let project_name = project["key"].as_str().ok_or("Failed to get project name")?;
        let all_repos = fetch_repositories(client, auth_header, repos_url_template, project_name)?;
        for repo in all_repos {
            let repo_name = repo["name"].as_str().ok_or("Missing repo name")?;
            run_analysis(client, auth_header, project_name, repo_name)?;
        }
    }
    Ok(())
}

