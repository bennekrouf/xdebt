
use std::error::Error;
// use serde_json::Value;

use crate::plugins::analyze_one_repo::analyze_one_repo;
use crate::create_config::AppConfig;
use crate::kpi::compute_kpi::compute_kpi;
use crate::models::KPIResult;

pub fn run_analysis(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<(), Box<dyn Error>> {
    if repo_name.ends_with("-configuration") || repo_name.ends_with("-tests") {
        return Ok(());
    }

    // Perform the analysis
    match analyze_one_repo(config, project_name, repo_name) {
        Ok(analysis_results) => {
            tracing::info!("Project: {}, Repo: {}", project_name, repo_name);
            tracing::info!("Analysis result: {}", serde_json::to_string_pretty(&analysis_results)?);

            // Compute KPIs
            let kpi_results: Vec<KPIResult> = analysis_results
                .iter()
                .map(|analysis| compute_kpi(analysis))
                .collect();

            // Display KPIs instead of appending to file or CSV
            for kpi in &kpi_results {
                tracing::info!("KPI Result: {}", serde_json::to_string_pretty(kpi)?);
            }
        }
        Err(e) => {
            tracing::error!("Failed to generate analysis JSON for project '{}', repo '{}': {}", project_name, repo_name, e);
        }
    }

    Ok(())
}

