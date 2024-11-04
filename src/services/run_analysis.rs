use serde_json::{json, Map, Value};
use std::error::Error;

use crate::kpi::compute_kpi::compute_kpi;
use crate::models::AppConfig;
use crate::models::KPIResult;
use crate::plugins::analyze_one_repo::analyze_one_repo;
use crate::utils::append_json_to_file::append_json_to_file;
use crate::utils::remove_null_values::remove_null_values;

pub fn run_analysis(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<(), Box<dyn Error>> {
    tracing::info!("starting analysis for : {}", repo_name);
    if repo_name.ends_with("-configuration") || repo_name.ends_with("-tests") {
        return Ok(());
    }

    // Perform the analysis
    match analyze_one_repo(config, project_name, repo_name) {
        Ok(analysis_results) => {
            tracing::info!("Project: {}, Repo: {}", project_name, repo_name);
            tracing::debug!(
                "Analysis result: {}",
                serde_json::to_string_pretty(&analysis_results)?
            );

            // Compute KPIs based on the analysis results
            let kpi_results: Vec<KPIResult> = analysis_results
                .iter()
                .filter_map(|analysis| compute_kpi(analysis)) // Filter out None values
                .collect();

            // Log KPIs
            for kpi in &kpi_results {
                tracing::info!("KPI Result: {}", serde_json::to_string_pretty(kpi)?);
            }

            // Only proceed if kpi_results is not empty
            if !kpi_results.is_empty() {
                // Group the KPI results under the repository name
                let mut grouped_results = Map::new();
                grouped_results.insert("repository_name".to_string(), json!(repo_name));
                grouped_results.insert("debt".to_string(), json!(kpi_results));

                let mut json_data = Value::Object(grouped_results);
                remove_null_values(&mut json_data); // Remove null entries

                tracing::info!("Json result: {}", &json_data);
                // Append the grouped KPI JSON to the file
                append_json_to_file(config, project_name, &json_data)?;
            } else {
                tracing::info!(
                    "No KPIs to record for project: {}, repo: {}",
                    project_name,
                    repo_name
                );
            }
        }
        Err(e) => {
            tracing::trace!(
                "Failed to generate analysis JSON for project '{}', repo '{}': {}",
                project_name,
                repo_name,
                e
            );
        }
    }

    Ok(())
}
