
use std::collections::HashMap;
use std::error::Error;
use serde_json::{json, Value};

pub fn analyze_package_json_content(
    app_name: &str,
    package_json: &Value,
    version_keywords: &[&str],
    reference_keywords: &[&str]
) -> Result<Value, Box<dyn Error>> {
    // Define equivalences for version_keywords
    let mut equivalences: HashMap<&str, Vec<&str>> = HashMap::new();
    equivalences.insert("angular", vec!["@angular/core", "angular"]);
    
    let mut versions = HashMap::new();
    let mut references = Vec::new();

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

    // Check for references in the content
    for &keyword in reference_keywords {
        if package_json_str.contains(keyword) {
            references.push(keyword.to_string());
        }
    }

    // If no version found for any of the keywords, push a default reference
    // if versions.is_empty() {
    //     references.push("No Angular or AngularJS matches found".to_string());
    // }

    // Build the JSON output
    let result = json!({
        "repository": app_name,
        "versions": versions,
        "references": references
    });

    Ok(result)
}

