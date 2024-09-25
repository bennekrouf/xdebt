
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use serde_json::{Value, Map};
use std::error::Error;
use tracing::{info, debug};

use crate::plugins::maven::analyze_pom_content::analyze_pom_content;
use crate::utils::download_xml_file::download_xml_file;
use crate::plugins::maven::run_maven_effective_pom::run_maven_effective_pom;
use crate::models::AppConfig;
use crate::plugins::maven::parse_pom_for_modules::parse_pom_for_modules;

fn remove_dtd(xml_content: &str) -> String {
    // Remove DTD declaration if it exists
    xml_content
        .lines()
        .filter(|line| !line.trim().starts_with("<!DOCTYPE"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn process_pom(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    target_folder: &str,
    pom_url: &str,
    versions_keywords: &[&str],
) -> Result<Map<String, Value>, Box<dyn Error>> {
    let pom_file_path: PathBuf = Path::new(target_folder).join("pom.xml");

    // Check if the POM file already exists and handle FORCE_GIT_PULL
    if pom_file_path.exists() && !&config.force_git_pull {
        info!("POM file '{}' already exists, skipping download.", pom_file_path.display());
    } else {
        info!("Downloading POM file from '{}'", pom_url);
        let result = download_xml_file(config, pom_url, target_folder, "pom.xml");

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

    // Remove DTD if present
    let main_pom_content_no_dtd = remove_dtd(&main_pom_content);

    // Parse the POM content for modules and download their POMs before running maven
    let modules = parse_pom_for_modules(&main_pom_content_no_dtd)?;

    if !modules.is_empty() {
        info!("Multi-module POM detected. Modules: {:?}", modules);

        for module in modules {
            let module_pom_url = config.url_config.raw_file_url(
                project_name,
                repo_name,
                &module);
            info!("module_pom_url {}", module_pom_url);

            let module_target_folder = Path::new(target_folder).join(&module);
            info!("Downloading POM for module '{}' from URL: {}", module, module_pom_url);
            // Create a folder for each module
            std::fs::create_dir_all(&module_target_folder)
                .map_err(|e| format!("Failed to create directory for module '{}': {}", module, e))?;

            // Download the module's POM using the corrected URL structure
            download_xml_file(
                config,
                &module_pom_url,
                module_target_folder.to_str().unwrap(),
                "pom.xml"
            )?;

            debug!("Module POM for '{}' downloaded successfully.", module);
        }

    }

    // After downloading all POM files, run 'maven effective-pom'
    let mut pom_versions = Map::new();
    let effective_pom_file = Path::new(target_folder).join("effective_pom.xml");

    // Check if the effective POM exists or force_maven_effective is true
    if !effective_pom_file.exists() || config.force_maven_effective {
        info!(
            "Effective POM file '{}' does not exist or force_effective is true, generating effective POM.",
            effective_pom_file.display()
        );

        // Run 'maven effective-pom'
        let effective_pom_result = run_maven_effective_pom(&pom_file_path.to_string_lossy())?;
        let effective_pom_path = Path::new(&effective_pom_result);

        // Adjust the check for the effective POM path
        if !effective_pom_path.exists() {
            return Err(format!("Effective POM file '{}' does not exist.", effective_pom_path.display()).into());
        }

        let mut content = String::new();
        File::open(&effective_pom_path)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| format!("Failed to read effective POM file '{}': {}", effective_pom_path.display(), e))?;

        // Analyze the POM content
        let pom_analysis_result = analyze_pom_content(config, repo_name, &content, versions_keywords)?;
        debug!("analyze_pom_content returns {}", pom_analysis_result);

        pom_versions.extend(pom_analysis_result.get("versions").and_then(Value::as_object).unwrap_or(&Map::new()).clone());

        return Ok(pom_versions);
    } else {
        info!(
            "Effective POM file '{}' already exists, skipping generation.",
            effective_pom_file.display()
        );
    }

    Ok(pom_versions)
}

