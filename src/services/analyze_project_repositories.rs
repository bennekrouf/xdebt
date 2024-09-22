
use std::error::Error;
use dialoguer::Input;

use crate::utils::fetch_repositories::fetch_repositories;
use crate::services::run_analysis::run_analysis;
use crate::models::AppConfig;

pub fn analyze_project_repositories(
    config: &AppConfig,
) -> Result<(), Box<dyn Error>> {

    let project_name: String = Input::new()
        .with_prompt("Enter the project name (e.g., PTEP):")
        .interact()?;

    let all_repos = fetch_repositories(config, &project_name)?;

    for repo in all_repos {
        if let Some(repo_obj) = repo.as_object() {
            let repo_name = repo_obj.get("name")
                .and_then(|v| v.as_str())
                .ok_or("Missing repo name")?;

            run_analysis(config, &project_name, repo_name)?;
        } else {
            tracing::error!("Invalid repository format for project '{}'", project_name);
            return Err("Invalid repository format".into());
        }
    }

    Ok(())
}

