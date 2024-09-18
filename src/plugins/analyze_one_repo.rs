
use std::error::Error;
use std::env;
use serde_json::{Value, Map};
use reqwest::header::HeaderValue;
use tracing::{info, warn, error, debug, trace};

use crate::plugins::npm::analyze_package_json_content::analyze_package_json_content;
use crate::plugins::maven::process_pom::process_pom;
use crate::plugins::docker::check_dockerfile_exists::check_dockerfile_exists;
use crate::plugins::dotnet::check_csproj_files::check_csproj_files;
use crate::plugins::php::check_php_files::check_php_files;
use crate::plugins::jenkins::check_jenkins_file_exists::check_jenkins_file_exists;
use crate::plugins::jenkins::extract_version_from_groovy::extract_version_from_groovy;
use crate::utils::enrich_versions_with_roadmap::enrich_versions_with_roadmap;
use crate::services::get_distinct_products::get_distinct_products;
use crate::create_config::AppConfig;

pub fn analyze_one_repo(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = &config.client;
    let auth_header = &config.auth_header;
    let url_config = &*config.url_config; // Dereference the Box
    let db = &config.db;

    // Get the target folder from the environment
    let target_folder = env::var("TARGET_FOLDER")
        .unwrap_or_else(|_| "tmp".to_string());  // Default to "tmp" if not set
    let target_folder = format!("{}/{}/{}", &target_folder, &project_name, &repo_name);

    // Get FORCE_GIT_PULL from .env
    let force_git_pull = env::var("FORCE_GIT_PULL")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false); // Default to `false` if parsing fails

    // Fetch distinct product names from the Sled DB
    let product_names = get_distinct_products(db)?;

    // Convert Vec<String> to Vec<&str>
    let versions_keywords: Vec<&str> = product_names.iter().map(|s| s.as_str()).collect();

    // Get POM URL using UrlConfig
    let pom_url = url_config.file_url(project_name, repo_name, "pom.xml");

    // Try to process POM and continue even if there's an error
    let mut pom_versions = match process_pom(
        config, repo_name, &target_folder, &pom_url, &versions_keywords, force_git_pull
    ) {
        Ok(versions) => versions,
        Err(e) => {
            warn!("Failed to generate POM analysis for project '{}', repo '{}': {}", project_name, repo_name, e);
            // Insert empty versions if POM analysis fails
            Map::new()
        }
    };

    // Initialize the final result JSON
    let mut final_result = Map::new();

    // Get package.json URL using UrlConfig
    let package_json_url = url_config.package_json_url(project_name, repo_name);

    info!("Fetching package.json from URL: {}", package_json_url);

    let pkg_response = client.get(&package_json_url)
        .header("Authorization", HeaderValue::from_str(auth_header)?)
        .header("Content-Type", "application/json")
        .send()?;

    if pkg_response.status().is_success() {
        let pkg_content = pkg_response.text()?;
        let package_json: Value = serde_json::from_str(&pkg_content)?;

        let package_json_analysis_result = analyze_package_json_content(
            repo_name, &package_json, &versions_keywords
            )?;
        pom_versions.extend(package_json_analysis_result
            .get("versions")
            .and_then(Value::as_object)
            .unwrap_or(&Map::new())
            .clone());

        if !pom_versions.is_empty() {
            final_result.insert("versions".to_string(), Value::Object(pom_versions));
        }

        if let Some(references) = package_json_analysis_result.get("references").cloned() {
            if !references.as_array().unwrap_or(&Vec::new()).is_empty() {
                final_result.insert("references".to_string(), references);
            }
        }

        final_result.insert("repository".to_string(), Value::String(repo_name.to_string()));
    } else if pkg_response.status() == 404 {
        warn!("package.json not found (HTTP 404), continuing without it.");
        final_result.insert("repository".to_string(), Value::String(repo_name.to_string()));
        if !pom_versions.is_empty() {
            final_result.insert("versions".to_string(), Value::Object(pom_versions));
        }
    } else {
        error!("Failed to fetch package.json: HTTP {}", pkg_response.status());
    }

    // Check if Dockerfile exists in the repository
    let dockerfile_exists = check_dockerfile_exists(config, project_name, repo_name)?;
    if dockerfile_exists {
        final_result.insert("Docker".to_string(), Value::Bool(dockerfile_exists));
    }

    // Check if .csproj exists in the repository
    let csproj_exists = check_csproj_files(config, project_name, repo_name)?;
    if csproj_exists {
        final_result.insert("C#".to_string(), Value::Bool(csproj_exists));
    }

    // Check if PHP repository files exist
    let php_files_exists = check_php_files(config, project_name, repo_name)?;
    if php_files_exists {
        final_result.insert("php".to_string(), Value::Bool(php_files_exists));
    }

    // Jenkins analysis: Check if Jenkins file exists
    if let Some(jenkins_file_url) = check_jenkins_file_exists(config, project_name, repo_name)? {
        info!("Found Jenkins file at: {}", jenkins_file_url);
        let jenkins_file_content = client.get(&jenkins_file_url)
            .header("Authorization", HeaderValue::from_str(auth_header)?)
            .send()?
            .text()?;

        // Extract versions for each keyword from the Jenkins file
        for keyword in &versions_keywords {
            if let Some(version) = extract_version_from_groovy(&jenkins_file_content, keyword) {
                // Insert the version into the "versions" object
                final_result.entry("versions".to_string())
                    .or_insert_with(|| Value::Object(Map::new()))
                    .as_object_mut()
                    .unwrap()
                    .insert(keyword.to_string(), Value::String(version));
            } else {
                trace!("No version found for keyword '{}' in Jenkins file.", keyword);
            }
        }
    } else {
        info!("No Jenkins file found for project '{}', repo '{}'.", project_name, repo_name);
    }

    debug!("Final result of analysis for project '{}', repo '{}': {:?}", project_name, repo_name, final_result);

    // Extract the "versions" object from the result
    if let Some(versions) = final_result.get("versions").and_then(Value::as_object) {
        // Call the transform function
        let transformed_versions = enrich_versions_with_roadmap(&db, versions)?;

        // Update the final_result with the transformed versions
        final_result.insert("versions".to_string(), Value::Object(transformed_versions));

        debug!("Updated result: {:?}", final_result);
    }

    Ok(Value::Object(final_result))
}

