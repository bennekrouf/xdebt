use std::collections::HashMap;
use serde_json::json;
use tokio::io::{self, BufReader};
use tokio::io::AsyncBufReadExt;
use crate::fetch_repositories::fetch_repositories;
use crate::services::get_projects::get_projects;
use crate::services::run_analysis::run_analysis;
use crate::models::AppConfig;
use crate::utils::append_json_to_file::append_json_to_file;
use crate::types::{MyError, CustomError};

pub async fn analyze_specific_repository(
    config: &AppConfig,
    repo_name_arg: Option<&str>,
) -> Result<(), MyError> {
    // Get repository name
    let repo_name = match repo_name_arg {
        Some(name) => name.to_string(),
        None => {
            let mut input = String::new();
            let mut stdin = BufReader::new(io::stdin());
            stdin.read_line(&mut input)
                .await
                .map_err(|e| CustomError::IoError(e))?;
            input.trim().to_string()
        }
    };

    if repo_name.is_empty() {
        return Err(CustomError::InvalidInput("Repository name cannot be empty".to_string()).into());
    }

    // Fetch all projects
    let projects = get_projects(config)
        .await
        .map_err(|e| CustomError::ProjectError(e.to_string()))?;

    let mut repository_found = false;
    let mut all_analysis_results: HashMap<String, Vec<serde_json::Value>> = HashMap::new();

    for project in projects {
        let project_name = project["key"]
            .as_str()
            .ok_or_else(|| CustomError::ProjectError("Failed to get project name".to_string()))?;

        let mut project_analysis_results = Vec::new();

        // Fetch repositories for the project
        let all_repos = fetch_repositories(config, project_name)
            .await
            .map_err(|e| CustomError::ProjectError(format!("Failed to fetch repositories: {}", e)))?;

        for repo in all_repos {
            let repo_actual_name = repo["name"]
                .as_str()
                .ok_or_else(|| CustomError::NotFound("Missing repo name".to_string()))?;

            if repo_actual_name == repo_name {
                repository_found = true;
                // Run analysis
                if let Some(json_data) = run_analysis(config, project_name, &repo_name)
                    .await
                    .map_err(|e| CustomError::AnalysisFailed(e.to_string()))? {
                    project_analysis_results.push(json_data);
                }
                break;
            }
        }

        // If we found results for this project, add them to the HashMap
        if !project_analysis_results.is_empty() {
            all_analysis_results.insert(project_name.to_string(), project_analysis_results.clone());
            
            // Create the nested structure for this project
            let project_result = HashMap::from([(
                project_name.to_string(),
                project_analysis_results
            )]);
            
            // Convert to JSON and save
            let json_project_result = json!(project_result);
            append_json_to_file(config, project_name, &json_project_result)
                .map_err(|e| CustomError::project_error(format!("Failed to write results: {}", e)))?;
        }
    }

    if !repository_found {
        return Err(CustomError::NotFound(format!("Repository '{}' not found in any project", repo_name)).into());
    }

    Ok(())
}
