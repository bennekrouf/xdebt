
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use serde_json::{Value, Map};
use std::error::Error;
use tracing::{trace,info,debug};

use crate::plugins::maven::analyze_pom_content::analyze_pom_content;
use crate::utils::download_file::download_file;
use crate::utils::run_maven_effective_pom::run_maven_effective_pom;

pub fn process_pom(
    client: &Client,
    auth_header: &str,
    repo_name: &str,
    target_folder: &str,
    pom_url: &str,
    versions_keywords: &[&str],
    reference_keywords: &[&str],
    force_git_pull: bool,
) -> Result<Map<String, Value>, Box<dyn Error>> {
    let pom_file_path: PathBuf = Path::new(target_folder).join("pom.xml");

    // Check if the POM file already exists and handle FORCE_GIT_PULL
    if pom_file_path.exists() && !force_git_pull {
        info!("POM file '{}' already exists, skipping download.", pom_file_path.display());
    } else {
        info!("Downloading POM file from '{}'", pom_url);
        let result = download_file(&client, &auth_header, pom_url, target_folder, "pom.xml");

        // If the POM file returns a 404, log and return an empty result
        if let Err(e) = result {
            if e.to_string().contains("404 Not Found") {
                return Err(format!("POM file not found at URL '{}'. Skipping repository '{}'.", pom_url, repo_name).into());
            } else {
                return Err(format!("Error while downloading POM file: {}", e).into());
            }
        }
    }

    // Read and parse the main POM file
    let mut main_pom_content = String::new();
    File::open(&pom_file_path)
        .and_then(|mut file| file.read_to_string(&mut main_pom_content))
        .map_err(|e| format!("Failed to read main POM file '{}': {}", pom_file_path.display(), e))?;

    // Parse the POM content for modules and download their POMs before running maven
    let modules = parse_pom_for_modules(&main_pom_content)?;

    if !modules.is_empty() {
        info!("Multi-module POM detected. Modules: {:?}", modules);

        for module in modules {
            let module_pom_url = format!("{}/{}", pom_url.trim_end_matches("/pom.xml?raw"), module);
            let module_pom_url = format!("{}/pom.xml", module_pom_url);
            let module_target_folder = Path::new(target_folder).join(&module);

            debug!("Downloading POM for module '{}' from URL: {}", module, module_pom_url);
            debug!("Target folder for module '{}': {}", module, module_target_folder.display());

            // Create a folder for each module
            std::fs::create_dir_all(&module_target_folder)
                .map_err(|e| format!("Failed to create directory for module '{}': {}", module, e))?;

            // Download the module's POM as 'pom.xml'
            let download_url = format!("{}?raw", module_pom_url);
            debug!("Executing download for URL: {}", download_url);

            download_file(
                &client,
                &auth_header,
                &download_url,
                module_target_folder.to_str().unwrap(),
                "pom.xml"
            )?;

            info!("Module POM for '{}' downloaded successfully.", module);
        }
    }

    // After downloading all POM files, run 'maven effective-pom'
    let effective_pom_result = run_maven_effective_pom(&pom_file_path.to_string_lossy(), repo_name);

    let mut pom_versions = Map::new();
    if let Ok(effective_pom_file) = effective_pom_result {
        let effective_pom_path = Path::new(target_folder).join(&effective_pom_file);
        if !effective_pom_path.exists() {
            return Err(format!("Effective POM file '{}' does not exist.", effective_pom_path.display()).into());
        }

        let mut content = String::new();
        File::open(&effective_pom_path)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| format!("Failed to read effective POM file '{}': {}", effective_pom_path.display(), e))?;

        // Analyze the POM content
        let pom_analysis_result = analyze_pom_content(repo_name, &content, versions_keywords, reference_keywords)?;
        info!("analyze_pom_content returns {}", pom_analysis_result);

        pom_versions.extend(pom_analysis_result.get("versions").and_then(Value::as_object).unwrap_or(&Map::new()).clone());
    }

    Ok(pom_versions)
}


// Function to parse the POM content for modules
fn parse_pom_for_modules(pom_content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    trace!("Parsing POM content for modules");
    let doc = roxmltree::Document::parse(pom_content)
        .map_err(|e| format!("Failed to parse POM XML: {}", e))?;

    let mut modules = Vec::new();
    for node in doc.descendants() {
        if node.tag_name().name() == "module" {
            if let Some(module_name) = node.text() {
                modules.push(module_name.to_string());
            }
        }
    }
    Ok(modules)
}

