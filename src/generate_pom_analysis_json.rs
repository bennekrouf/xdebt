
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use base64::{engine::general_purpose, Engine as _};
use std::env;
use std::path::{Path, PathBuf};
use serde_json::{Value, Map};

use crate::analyze_pom_content::analyze_pom_content;
use crate::analyze_package_json_content::analyze_package_json_content;
use crate::download_file::download_file;
use crate::run_maven_effective_pom::run_maven_effective_pom;

pub fn generate_pom_analysis_json(
    project_name: &str,
    repo_name: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
    // Get the target folder from the environment
    let target_folder = env::var("TARGET_FOLDER")
        .unwrap_or_else(|_| "tmp".to_string());  // Default to "tmp" if not set
    let target_folder = format!("{}/{}", &target_folder, &project_name);

    // Initialize reqwest client
    let client = Client::new();

    // Get credentials from environment variables
    let username = env::var("BITBUCKET_USERNAME")
        .map_err(|e| format!("Missing BITBUCKET_USERNAME environment variable: {}", e))?;
    let password = env::var("BITBUCKET_PASSWORD")
        .map_err(|e| format!("Missing BITBUCKET_PASSWORD environment variable: {}", e))?;
    let auth_header = format!(
        "Basic {}",
        general_purpose::STANDARD.encode(format!("{}:{}", username, password))
    );

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

    // Determine the download path for the POM file
    let pom_file_path: PathBuf = Path::new(&target_folder).join(format!("{}_pom.xml", repo_name));

    // Check if the POM file already exists and handle FORCE_REFRESH
    if pom_file_path.exists() && !force_refresh {
        println!("POM file '{}' already exists, skipping download.", pom_file_path.display());
    } else {
        // Download the file if it doesn't exist or if FORCE_REFRESH is true
        // println!("Downloading POM file from URL: {}", pom_url);
        download_file(&client, &auth_header, &pom_url, &target_folder, repo_name)
            .map_err(|e| format!("Error while downloading POM file: {}", e))?;
    }

    // Run Maven on the downloaded (or existing) POM file
    let effective_pom_result = run_maven_effective_pom(&pom_file_path.to_string_lossy(), &repo_name);

    // Initialize the final result JSON
    let mut final_result = Map::new();

    let mut pom_versions = Map::new();
    if let Ok(effective_pom_file) = effective_pom_result {
        let effective_pom_path = Path::new(&target_folder).join(&effective_pom_file);
        if !effective_pom_path.exists() {
            return Err(format!("Effective POM file '{}' does not exist.", effective_pom_path.display()).into());
        }

        let mut file = File::open(&effective_pom_path)
            .map_err(|e| format!("Failed to open effective POM file '{:?}': {}", effective_pom_path, e))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| format!("Failed to read content of effective POM file '{:?}': {}", effective_pom_path, e))?;

        let pom_analysis_result = analyze_pom_content(repo_name, &content, &versions_keywords, &reference_keywords)?;
        println!("analyze_pom_content returns {}", pom_analysis_result);

        pom_versions.extend(pom_analysis_result.get("versions").and_then(Value::as_object).unwrap_or(&Map::new()).clone());
    }

    // Get package.json URL from .env
    let package_json_url = env::var("PACKAGE_JSON_URL")
        .map_err(|e| format!("Missing PACKAGE_JSON_URL environment variable: {}", e))?;

    let package_json_url = package_json_url
        .replace("{project_name}", project_name)
        .replace("{repo_name}", repo_name);

    println!("Fetching package.json from URL: {}", package_json_url);

    let pkg_response = client.get(&package_json_url)
        .header("Authorization", &auth_header)
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

