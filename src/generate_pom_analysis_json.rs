
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use base64::{engine::general_purpose, Engine as _};
use std::env;
use std::path::{Path, PathBuf};
// use serde_json::Value;

use crate::analyze_pom_content::analyze_pom_content;
use crate::download_file::download_file;
use crate::run_maven_effective_pom::run_maven_effective_pom;

pub fn generate_pom_analysis_json(
    project_name: &str,
    repo_name: &str,
    target_folder: &str,
) -> Result<serde_json::Value, Box<dyn Error>> {
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

    // Get base URL from .env
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

    // Replace placeholders in URL
    let url = base_url
        .replace("{project_name}", project_name)
        .replace("{repo_name}", repo_name);

    // Determine the download path for the POM file
    let pom_file_path: PathBuf = Path::new(target_folder).join(format!("{}_pom.xml", repo_name));

    // Check if the POM file already exists and handle FORCE_REFRESH
    if pom_file_path.exists() && !force_refresh {
        println!("POM file '{}' already exists, skipping download.", pom_file_path.display());
    } else {
        // Download the file if it doesn't exist or if FORCE_REFRESH is true
        println!("Downloading POM file from URL: {}", url);
        download_file(&client, &auth_header, &url, target_folder, repo_name)
            .map_err(|e| format!("Error while downloading POM file: {}", e))?;
    }

    // Run Maven on the downloaded (or existing) POM file
    let effective_pom_result = run_maven_effective_pom(&pom_file_path.to_string_lossy(), &repo_name);

    // Handle the result of Maven command execution
    match effective_pom_result {
        Ok(effective_pom_file) => {
            // Construct the full path for the effective POM file
            let effective_pom_path = Path::new(target_folder).join(&effective_pom_file);

            // Check if the effective POM file exists
            if !effective_pom_path.exists() {
                return Err(format!("Effective POM file '{}' does not exist.", effective_pom_path.display()).into());
            }

            // Read the effective POM file content
            let mut file = File::open(&effective_pom_path)
                .map_err(|e| format!("Failed to open effective POM file '{:?}': {}", effective_pom_path, e))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| format!("Failed to read content of effective POM file '{:?}': {}", effective_pom_path, e))?;

            // Analyze the POM content and generate JSON output
            analyze_pom_content(repo_name, &content, &reference_keywords)
        },
        Err(e) => {
            eprintln!("Failed to run Maven effective POM: {}", e);
            Err(e.into()) // Return the error directly
        }
    }
}

