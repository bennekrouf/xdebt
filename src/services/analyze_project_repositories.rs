
use std::error::Error;
use dialoguer::Input;

use crate::utils::fetch_repositories::fetch_repositories;
use crate::services::run_analysis::run_analysis;

pub fn analyze_project_repositories(
    db: &sled::Db,
    client: &reqwest::blocking::Client,
    auth_header: &str,
    repos_url_template: &str
) -> Result<(), Box<dyn Error>> {
    let project_name: String = Input::new()
        .with_prompt("Enter the project name (e.g., PTEP):")
        .interact()?;

    let all_repos = fetch_repositories(client, auth_header, repos_url_template, &project_name)?;

    for repo in all_repos {
        if let Some(repo_obj) = repo.as_object() {
            let repo_name = repo_obj.get("name")
                .and_then(|v| v.as_str())
                .ok_or("Missing repo name")?;

            run_analysis(db, client, auth_header, &project_name, repo_name)?;
        } else {
            tracing::error!("Invalid repository format for project '{}'", project_name);
            return Err("Invalid repository format".into());
        }
    }

    Ok(())
}

