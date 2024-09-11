
use std::collections::HashMap;
use std::error::Error;
use serde_json::{json, Value};

pub fn analyze_package_json_content(
    app_name: &str,
    package_json: &Value
) -> Result<Value, Box<dyn Error>> {
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

    // Check in "dependencies"
    if let Some(dependencies) = package_json_value.get("dependencies") {
        if let Some(deps_obj) = dependencies.as_object() {
            if let Some(version) = &deps_obj.get("@angular/core").and_then(|v| v.as_str()) {
                let cleaned_version = version.trim_start_matches(['~', '^']);
                versions.insert("Angular".to_string(), cleaned_version.to_string());
            }
            if let Some(version) = &deps_obj.get("angular").and_then(|v| v.as_str()) {
                let cleaned_version = version.trim_start_matches(['~', '^']);
                versions.insert("AngularJS".to_string(), cleaned_version.to_string());
            }
        }
    }

    // Check in "devDependencies"
    if let Some(dev_dependencies) = package_json_value.get("devDependencies") {
        if let Some(dev_deps_obj) = dev_dependencies.as_object() {
            if let Some(version) = &dev_deps_obj.get("angular").and_then(|v| v.as_str()) {
                let cleaned_version = version.trim_start_matches(['~', '^']);
                versions.insert("AngularJS in Dev deps".to_string(), cleaned_version.to_string());
            }
            if let Some(version) = &dev_deps_obj.get("@angular/core").and_then(|v| v.as_str()) {
                let cleaned_version = version.trim_start_matches(['~', '^']);
                versions.insert("Angular in Dev deps".to_string(), cleaned_version.to_string());
            }
        }
    }

    // Check for references
    if versions.is_empty() {
        references.push("No Angular or AngularJS matches found".to_string());
    }

    // Build the JSON output
    let result = json!({
        "repository": app_name,
        "versions": versions,
        "references": references
    });

    Ok(result)
}

