
use std::error::Error;

use crate::plugins::analyze_one_repo::analyze_one_repo;
use crate::utils::append_json_to_file::append_json_to_file;
use crate::utils::append_json_to_csv::append_json_to_csv;
use crate::create_config::AppConfig;

pub fn run_analysis(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<(), Box<dyn Error>> {
    if repo_name.ends_with("-configuration") || repo_name.ends_with("-tests") {
        return Ok(());
    }

    match analyze_one_repo(config, project_name, repo_name) {
        Ok(json_result) => {
            tracing::info!("Project: {}, Repo: {}", project_name, repo_name);
            tracing::info!("Analysis result: {}", serde_json::to_string_pretty(&json_result)?);

            if let Err(e) = append_json_to_file(project_name, &json_result) {
                tracing::error!("Failed to append JSON to file for project '{}', repo '{}': {}", project_name, repo_name, e);
            }

            if let Err(e) = append_json_to_csv(project_name, &json_result) {
                tracing::error!("Failed to append JSON to CSV for project '{}', repo '{}': {}", project_name, repo_name, e);
            }
        }
        Err(e) => {
            tracing::error!("Failed to generate POM analysis JSON for project '{}', repo '{}': {}", project_name, repo_name, e);
        }
    }

    Ok(())
}

