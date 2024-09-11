
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use std::env;

mod download_file;
mod run_maven_effective_pom;
mod analyze_pom_content;

use crate::analyze_pom_content::analyze_pom_content;
use crate::download_file::download_file;
use crate::run_maven_effective_pom::run_maven_effective_pom;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();  // Load environment variables from .env file

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

    // Define the repository and project names
    let repo_name = "shared-ace-client";
    let project_name = "SHARED";

    // Construct the URL
    let url = format!(
        "https://dsigit.etat-de-vaud.ch/outils/git/projects/{}/repos/{}/browse/pom.xml?raw",
        project_name,
        repo_name
    );

    // Define the target folder
    let target_folder = "tmp";

    // Download the file
    let downloaded_file = download_file(&client, &auth_header, &url, target_folder, repo_name)
        .map_err(|e| format!("Error while downloading POM file: {}", e))?;

    // Run Maven on the downloaded POM file
    let effective_pom_file = run_maven_effective_pom(&downloaded_file, &repo_name)
        .map_err(|e| format!("Error running Maven on '{}': {}", downloaded_file, e))?;

    // Construct the full path for the effective POM file
    let effective_pom_path = Path::new(target_folder).join(&effective_pom_file);

    // Check if the effective POM file exists
    if !effective_pom_path.exists() {
        return Err(format!("Effective POM file '{}' does not exist.", effective_pom_path.display()).into());
    }

    // Check if the effective POM file exists
    if !Path::new(&effective_pom_path).exists() {
        return Err(format!("Effective POM file '{}' does not exist.", effective_pom_file).into());
    }

    // Read the effective POM file content
    let mut file = File::open(&effective_pom_path)
        .map_err(|e| format!("Failed to open effective POM file '{:?}': {}", effective_pom_path, e))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| format!("Failed to read content of effective POM file '{:?}': {}", effective_pom_path, e))?;

    // Analyze the content of the effective POM file
    let references = analyze_pom_content(&content)
        .map_err(|e| format!("Error analyzing effective POM content: {}", e))?;

    // Print references
    for reference in references {
        println!("{}", reference);
    }

    Ok(())
}

