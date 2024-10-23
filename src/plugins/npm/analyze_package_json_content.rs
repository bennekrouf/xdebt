
use std::collections::HashMap;
use crate::types::MyError;
use serde_json::{json, Value};
use tracing::{debug, info};

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
        .map(|cycle| cycle.trim_start_matches(['~', '^']).to_string())
}


pub async fn analyze_package_json_content(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    dependencies_list: &[&str],  // List of product names
) -> Result<Value, MyError> {
    // Check if package.json exists and get the file URL
    let file_url = match check_package_json_exists(config, project_name, repo_name).await? {
        Some(url) => url,
        None => {
            info!("No package.json found in the repository. Skipping analysis.");
            return Ok(json!({
                "repository": repo_name,
                "versions": {}  // Return an empty object or desired default value
            }));
        }
    };

    info!("Fetching package.json from URL: {}", file_url);

    // Fetch package.json content using the file URL
    let package_json: Value = run_json_get_query(config, &file_url).await?;

    let mut versions = HashMap::new();

    // There's no "lines" array, so directly work with the JSON object
    let package_json_value = package_json;

    // Extract dependencies and devDependencies
    let binding = json!({});
    let dependencies = package_json_value.get("dependencies").unwrap_or(&binding);
    let dev_dependencies = package_json_value.get("devDependencies").unwrap_or(&binding);

    // Log found dependencies and devDependencies
    debug!("Found dependencies: {:?}", dependencies);
    debug!("Found devDependencies: {:?}", dev_dependencies);

    // Loop through each product in the dependencies_list
    for product in dependencies_list {
        // Start with the product itself
        let mut keywords_to_check = vec![product.to_string()];

        // Check if the config has equivalences for this product
        if let Some(equivalences) = config.equivalences.get(*product) {
            // Extend with equivalences if they exist
            keywords_to_check.extend(equivalences.clone());
        }

        // Log the keywords that are being checked
        info!(
            "Checking for keywords for product '{}': {:?}",
            product, keywords_to_check
        );

        // Iterate over each keyword (product + equivalences)
        for kw in &keywords_to_check {
            // Log the search for each keyword in dependencies and devDependencies
            debug!("Looking for keyword '{}' in 'dependencies'", kw);
            if dependencies.get(kw).is_some() {
                info!("Found keyword '{}' in 'dependencies'", kw);
            }

            debug!("Looking for keyword '{}' in 'devDependencies'", kw);
            if dev_dependencies.get(kw).is_some() {
                debug!("Found keyword '{}' in 'devDependencies'", kw);
            }

            // Check in both dependencies and devDependencies
            if let Some(cycle) = get_dependency_version(dependencies, dev_dependencies, kw) {
                debug!(
                    "Found version '{}' for keyword '{}' in 'dependencies' or 'devDependencies'",
                    cycle, kw
                );
                versions.insert(product.to_string(), cycle);  // Use the original product name for insertion
                break; // Stop searching once a version is found
            } else {
                info!(
                    "Keyword '{}' not found in 'dependencies' or 'devDependencies'",
                    kw
                );
            }
        }
    }

    // Build the JSON output
    let result = json!({
        "repository": repo_name,
        "versions": versions,
    });

    info!("Result package json: {}", result);

    Ok(result)
}

