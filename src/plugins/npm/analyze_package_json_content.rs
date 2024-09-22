
use std::collections::HashMap;
use std::error::Error;
use serde_json::{json, Value};
use tracing::info;

use crate::models::AppConfig;
use crate::utils::run_get_query::run_get_query;

pub fn analyze_package_json_content(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    version_keywords: &[&str],
) -> Result<Value, Box<dyn Error>> {
    // Get package.json URL using UrlConfig
    let package_json_url = &config.url_config.package_json_url(project_name, repo_name);

    info!("Fetching package.json from URL: {}", package_json_url);

    // Use run_get_query to fetch package.json content
    let package_json: Value = run_get_query(config, package_json_url)?;

    // Define equivalences for version_keywords
    let mut equivalences: HashMap<&str, Vec<&str>> = HashMap::new();
    equivalences.insert("angular", vec!["@angular/core", "angular"]);

    let mut versions = HashMap::new();

    // Access the actual package.json content within the "lines" array
    let lines = package_json.get("lines")
        .and_then(|l| l.as_array())
        .ok_or("Invalid format: 'lines' is missing or not an array")?;

    // Collect all text lines into a single string
    let package_json_str: String = lines.iter()
        .filter_map(|line| line.get("text").and_then(|t| t.as_str()))
        .collect();

    // Parse the JSON string into a Value object
    let package_json_value: Value = serde_json::from_str(&package_json_str)?;

    // Check for versions based on version_keywords
    for keyword in version_keywords {
        if let Some(refs) = equivalences.get(keyword) {
            for &reference in refs {
                // Check in "dependencies"
                if let Some(dependencies) = package_json_value.get("dependencies") {
                    if let Some(deps_obj) = dependencies.as_object() {
                        if let Some(version) = deps_obj.get(reference).and_then(|v| v.as_str()) {
                            let cleaned_version = version.trim_start_matches(['~', '^']);
                            versions.insert(keyword.to_string(), cleaned_version.to_string());
                        }
                    }
                }
                // Check in "devDependencies"
                if let Some(dev_dependencies) = package_json_value.get("devDependencies") {
                    if let Some(dev_deps_obj) = dev_dependencies.as_object() {
                        if let Some(version) = dev_deps_obj.get(reference).and_then(|v| v.as_str()) {
                            let cleaned_version = version.trim_start_matches(['~', '^']);
                            versions.insert(format!("{} in Dev deps", keyword), cleaned_version.to_string());
                        }
                    }
                }
            }
        }

        // Check for direct keyword matches in dependencies and devDependencies
        if let Some(dependencies) = package_json_value.get("dependencies") {
            if let Some(deps_obj) = dependencies.as_object() {
                if let Some(version) = deps_obj.get(&keyword.to_string()).and_then(|v| v.as_str()) {
                    let cleaned_version = version.trim_start_matches(['~', '^']);
                    versions.insert(keyword.to_string(), cleaned_version.to_string());
                }
            }
        }
        if let Some(dev_dependencies) = package_json_value.get("devDependencies") {
            if let Some(dev_deps_obj) = dev_dependencies.as_object() {
                if let Some(version) = dev_deps_obj.get(&keyword.to_string()).and_then(|v| v.as_str()) {
                    let cleaned_version = version.trim_start_matches(['~', '^']);
                    versions.insert(format!("{} in Dev deps", keyword), cleaned_version.to_string());
                }
            }
        }
    }

    // Build the JSON output
    let result = json!({
        "repository": repo_name,
        "versions": versions,
    });

    Ok(result)
}

