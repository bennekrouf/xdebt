
use std::collections::HashMap;
use std::error::Error;
use serde_json::{json, Value};
use tracing::info;

use crate::models::AppConfig;
use crate::utils::run_json_get_query::run_json_get_query;
use crate::plugins::npm::check_package_json_exists::check_package_json_exists;

// Function to get the version from dependencies or devDependencies
fn get_dependency_version(
    dependencies: &Value,
    dev_dependencies: &Value,
    package_name: &str,
) -> Option<String> {
    dependencies.get(package_name)
        .or_else(|| dev_dependencies.get(package_name))
        .and_then(|v| v.as_str())
        .map(|version| version.trim_start_matches(['~', '^']).to_string())
}

pub fn analyze_package_json_content(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    dependencies_list: &[&str],  // List of dependency names
) -> Result<Value, Box<dyn Error>> {
    // Check if package.json exists and get the file URL
    let file_url = match check_package_json_exists(config, project_name, repo_name)? {
        Some(url) => url,
        None => return Err("No package.json found in the repository".into()),
    };

    info!("Fetching package.json from URL: {}", file_url);

    // Fetch package.json content using the file URL
    let package_json: Value = run_json_get_query(config, &file_url)?;

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

    // Extract dependencies and devDependencies
    let binding = json!({});
    let dependencies = package_json_value.get("dependencies").unwrap_or(&binding);
    let dev_dependencies = package_json_value.get("devDependencies").unwrap_or(&binding);

    // Loop through each dependency in the dependencies_list
    for dependency in dependencies_list {
        // Start with the dependency itself
        let mut keywords_to_check = vec![dependency.to_string()];

        // Check if the config has equivalences for this dependency
        if let Some(equivalences) = config.equivalences.get(*dependency) {
            // Extend with equivalences if they exist
            keywords_to_check.extend(equivalences.clone());
        }

        // Iterate over each keyword (dependency + equivalences)
        for kw in &keywords_to_check {
            // Check in both dependencies and devDependencies
            if let Some(version) = get_dependency_version(dependencies, dev_dependencies, kw) {
                versions.insert(dependency.to_string(), version);  // Use the original dependency name for insertion
                break; // Stop searching once a version is found
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

