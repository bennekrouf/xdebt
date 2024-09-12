
use reqwest::blocking::Client;
use std::error::Error;
use std::env;
use serde_json::{Value, Map};
use reqwest::header::HeaderValue;

use crate::utils::analyze_package_json_content::analyze_package_json_content;
use crate::process_pom::process_pom;

pub fn generate_analysis(
    client: &Client,
    auth_header: &str,
    project_name: &str,
    repo_name: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    // Get the target folder from the environment
    let target_folder = env::var("TARGET_FOLDER")
        .unwrap_or_else(|_| "tmp".to_string());  // Default to "tmp" if not set
    let target_folder = format!("{}/{}", &target_folder, &project_name);

    // Get POM URL from .env
    let base_url = env::var("BITBUCKET_POM_URL")
        .map_err(|e| format!("Missing BITBUCKET_POM_URL environment variable: {}", e))?;

    // Get FORCE_REFRESH from .env
    let force_refresh = env::var("FORCE_REFRESH")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false); // Default to `false` if parsing fails

    // Get REFERENCES array from .env
    let references_str = env::var("REFERENCES")
        .unwrap_or_else(|_| "jencks,nexus,xfile,php,richfaces".to_string());
    let reference_keywords: Vec<&str> = references_str.split(',').collect();

    // Get VERSIONS array from .env
    let versions_str = env::var("VERSIONS_OF")
        .unwrap_or_else(|_| "spring,java".to_string());
    let versions_keywords: Vec<&str> = versions_str.split(',').collect();

    // Replace placeholders in POM URL
    let pom_url = base_url
        .replace("{project_name}", project_name)
        .replace("{repo_name}", repo_name);

    // Process POM and retrieve the versions
    let mut pom_versions = process_pom(client, auth_header, repo_name, &target_folder, &pom_url, &versions_keywords, &reference_keywords, force_refresh)?;

    // Initialize the final result JSON
    let mut final_result = Map::new();

    // Get package.json URL from .env
    let package_json_url = env::var("PACKAGE_JSON_URL")
        .map_err(|e| format!("Missing PACKAGE_JSON_URL environment variable: {}", e))?;

    let package_json_url = package_json_url
        .replace("{project_name}", project_name)
        .replace("{repo_name}", repo_name);

    println!("Fetching package.json from URL: {}", package_json_url);

    let pkg_response = client.get(&package_json_url)
        .header("Authorization", HeaderValue::from_str(&auth_header)?)
        .header("Content-Type", "application/json")
        .send()?;

    if pkg_response.status().is_success() {
        let pkg_content = pkg_response.text()?;
        let package_json: Value = serde_json::from_str(&pkg_content)?;

        let package_json_analysis_result = analyze_package_json_content(repo_name, &package_json)?;
        pom_versions.extend(package_json_analysis_result.get("versions").and_then(Value::as_object).unwrap_or(&Map::new()).clone());

        final_result.insert("repository".to_string(), Value::String(repo_name.to_string()));
        final_result.insert("versions".to_string(), Value::Object(pom_versions));
        final_result.insert("references".to_string(), package_json_analysis_result.get("references").cloned().unwrap_or(Value::Array(Vec::new())));
    } else if pkg_response.status() == 404 {
        println!("package.json not found (HTTP 404), continuing without it.");
        // Insert only POM data in final_result
        final_result.insert("repository".to_string(), Value::String(repo_name.to_string()));
        final_result.insert("versions".to_string(), Value::Object(pom_versions));
        final_result.insert("references".to_string(), Value::Array(Vec::new())); // Empty references
    } else {
        eprintln!("Failed to fetch package.json: HTTP {}", pkg_response.status());
    }

    println!("Final result of generate pom analysis: {:?}", final_result);

    Ok(Value::Object(final_result))
}
