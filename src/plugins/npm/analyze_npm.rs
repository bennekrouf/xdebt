
use std::error::Error;
use serde_json::Value;
use tracing::info;

use crate::plugins::npm::analyze_package_json_content::analyze_package_json_content;
use crate::models::{AppConfig, Analysis};

pub fn analyze_npm(
    config: &AppConfig,
    project_name: &str,
    repository_name: &str,
    versions_keywords: &[&str],
    analyses: &mut Vec<Analysis>,
) -> Result<(), Box<dyn Error>> {
    info!("Analyzing package.json for repository: {}", repository_name);
    let package_json_analysis_result = analyze_package_json_content(config, project_name, repository_name, versions_keywords)?;

    let package_json_analyses: Vec<Analysis> = package_json_analysis_result
        .get("analyses")
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| serde_json::from_value::<Analysis>(v.clone()).ok())
        .collect();

    analyses.extend(package_json_analyses);
    Ok(())
}

