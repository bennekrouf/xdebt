
use std::time::{Instant, Duration};
use serde_json::{Value, json};
use crate::plugins::analyze_one_repo::analyze_one_repo;
use crate::models::AppConfig;
use crate::kpi::compute_kpi::compute_kpi;
use crate::models::KPIResult;
use crate::utils::remove_null_values::remove_null_values;
use crate::types::MyError;

static mut TOTAL_DURATION: Duration = Duration::new(0, 0);  // Static variable to hold total duration

pub fn run_analysis(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<Option<Value>, MyError> {
    // Skip certain repositories
    if repo_name.ends_with("-configuration") || repo_name.ends_with("-tests") {
        return Ok(None);  // Return None for skipped repos
    }

    // Start timing the analysis
    let start_time = Instant::now();

    // Perform the analysis
    match analyze_one_repo(config, project_name, repo_name) {
        Ok(mut analysis_results) => {
            tracing::info!("Project: {}, Repo: {}", project_name, repo_name);
            tracing::debug!("Analysis result: {}", serde_json::to_string_pretty(&analysis_results)?);

            // Compute KPIs based on the analysis results
            let kpi_results: Vec<KPIResult> = analysis_results
                .iter_mut()
                .filter_map(|analysis| compute_kpi(&config, analysis)) // Filter out None values
                .collect();

            // Log KPIs
            for kpi in &kpi_results {
                tracing::info!("KPI Result: {}", serde_json::to_string_pretty(kpi)?);
            }

            // Only proceed if kpi_results is not empty
            if !kpi_results.is_empty() {
                // Use a Vec to enforce field order
                let json_data = vec![
                    ("application", json!(repo_name)),  // Add the repo name first
                    ("debt", json!(kpi_results))            // Then the debt (KPI results)
                ];

                // Convert the Vec to a Value::Object by converting tuples to key-value pairs
                let mut flattened_json = serde_json::Map::new();
                for (key, value) in json_data {
                    flattened_json.insert(key.to_string(), value);
                }

                let mut final_json = Value::Object(flattened_json);
                remove_null_values(&mut final_json);  // Remove null entries

                tracing::debug!("Json result: {}", &final_json);

                // Log the duration of this analysis
                let duration = start_time.elapsed();
                tracing::info!(
                    "Analysis completed for project: {}, repo: {} in {:?}",
                    project_name,
                    repo_name,
                    duration
                );

                // Accumulate total duration
                unsafe {
                    TOTAL_DURATION += duration;
                }

                // Log the total accumulated time
                tracing::info!("Total time so far: {:?}", unsafe { TOTAL_DURATION });

                // Return the final JSON data
                return Ok(Some(final_json));
            } else {
                tracing::info!("No KPIs to record for project: {}, repo: {}", project_name, repo_name);

                // Log the duration of this analysis
                let duration = start_time.elapsed();
                tracing::info!(
                    "Analysis completed for project: {}, repo: {} in {:?} with no KPIs",
                    project_name,
                    repo_name,
                    duration
                );

                // Accumulate total duration
                unsafe {
                    TOTAL_DURATION += duration;
                }

                // Log the total accumulated time
                tracing::info!("Total time so far: {:?}", unsafe { TOTAL_DURATION });

                return Ok(None);  // Return None if no KPIs
            }
        }
        Err(e) => {
            tracing::trace!("Failed to generate analysis JSON for project '{}', repo '{}': {}", project_name, repo_name, e);
        }
    }

    // Log the duration in case of failure
    let duration = start_time.elapsed();
    tracing::info!(
        "Analysis failed for project: {}, repo: {} in {:?}",
        project_name,
        repo_name,
        duration
    );

    // Accumulate total duration
    unsafe {
        TOTAL_DURATION += duration;
    }

    // Log the total accumulated time
    tracing::info!("Total time so far: {:?}", unsafe { TOTAL_DURATION });

    Ok(None)
}

