
use std::error::Error;
use dialoguer::Input;
use serde_json::json; // Ensure this is imported
use crate::utils::fetch_repositories::fetch_repositories;
use crate::services::run_analysis::run_analysis;
use crate::models::AppConfig;
use crate::utils::append_json_to_file::append_json_to_file;

pub fn analyze_project_repositories(
    config: &AppConfig,
) -> Result<(), Box<dyn Error>> {

    // Prompt for the project name
    let project_name: String = Input::new()
        .with_prompt("Enter the project name (e.g., PTEP):")
        .interact()?;

    // Fetch all repositories for the given project
    let all_repos = fetch_repositories(config, &project_name)?;

    // Initialize a vector to accumulate analysis results for all repositories
    let mut project_analysis_results = Vec::new();

    // Iterate over all repositories
    for repo in all_repos {
        if let Some(repo_obj) = repo.as_object() {
            let repo_name = repo_obj.get("name")
                .and_then(|v| v.as_str())
                .ok_or("Missing repo name")?;

            // Run the analysis for the repository
            if let Some(json_data) = run_analysis(config, &project_name, repo_name)? {
                // Accumulate the analysis result
                project_analysis_results.push(json_data);
            }
        } else {
            tracing::error!("Invalid repository format for project '{}'", project_name);
            return Err("Invalid repository format".into());
        }
    }

    // After processing all repositories, append the accumulated results to a JSON file
    if !project_analysis_results.is_empty() {
        // Create a JSON array from the accumulated results
        let json_project_result = json!(project_analysis_results);

        // Append the JSON data to the file for the project
        append_json_to_file(config, &project_name, &json_project_result)?;
    }

    Ok(())
}

