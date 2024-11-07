use std::collections::HashMap;
use dialoguer::Input;
use serde_json::json;
use crate::fetch_repositories::fetch_repositories;
use crate::services::run_analysis::run_analysis;
use crate::models::AppConfig;
use crate::utils::append_json_to_file::append_json_to_file;
use crate::types::MyError;

pub async fn analyze_specific_project(
    config: &AppConfig,
) -> Result<(), MyError> {
    // Prompt for the project name
    let project_name: String = Input::new()
        .with_prompt("Enter the project name (e.g., PTEP):")
        .interact()?;

    // Initialize a HashMap to store the project's analysis results
    let mut all_analysis_results: HashMap<String, Vec<serde_json::Value>> = HashMap::new();

    // Fetch all repositories for the given project
    let all_repos = fetch_repositories(config, &project_name).await?;

    // Initialize a vector to accumulate analysis results for all repositories
    let mut project_analysis_results = Vec::new();

    // Iterate over all repositories
    for repo in all_repos {
        if let Some(repo_obj) = repo.as_object() {
            let repo_name = repo_obj.get("name")
                .and_then(|v| v.as_str())
                .ok_or("Missing repo name")?;

            // Run the analysis for the repository
            if let Some(json_data) = run_analysis(config, &project_name, repo_name).await? {
                // Accumulate the analysis result
                project_analysis_results.push(json_data);
            }
        } else {
            tracing::error!("Invalid repository format for project '{}'", project_name);
            return Err("Invalid repository format".into());
        }
    }

    // After processing all repositories, create the nested structure and save
    if !project_analysis_results.is_empty() {
        // Insert the results into the HashMap with the project name as the key
        all_analysis_results.insert(project_name.clone(), project_analysis_results);

        // Create the final JSON structure
        let json_result = json!(all_analysis_results);

        // Append the JSON data to the file for the project
        append_json_to_file(config, &project_name, &json_result)?;
    }

    Ok(())
}
