
use std::path::Path;
use serde_json::{Value, Map};
use std::error::Error;
use tracing::{info, debug};

use crate::utils::download_xml_file::download_xml_file;
use crate::models::AppConfig;
use crate::plugins::maven::parse_pom_for_modules::parse_pom_for_modules;
use crate::plugins::maven::generate_and_analyze_effective_pom::generate_and_analyze_effective_pom;
use crate::plugins::maven::download_and_read_pom::download_and_read_pom;

/// Main processing function to orchestrate the POM processing.
pub fn process_pom(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    output_folder: &str,
    pom_url: &str,
    versions_keywords: &[&str],
) -> Result<Map<String, Value>, Box<dyn Error>> {
    // Step 1: Download and read the POM
    let main_pom_content = download_and_read_pom(config, output_folder, pom_url, repo_name)?;

    // Step 2: Parse the POM and extract modules
    let modules = parse_pom_for_modules(&main_pom_content)?;
    if !modules.is_empty() {
        info!("Multi-module POM detected. Modules: {:?}", modules);

        for module in modules {
            let module_pom_url = config.url_config.raw_file_url(project_name, repo_name, &module);
            info!("module_pom_url {}", module_pom_url);

            let module_target_folder = Path::new(output_folder).join(&module);
            std::fs::create_dir_all(&module_target_folder)
                .map_err(|e| format!("Failed to create directory for module '{}': {}", module, e))?;

            download_xml_file(
                config,
                &module_pom_url,
                module_target_folder.to_str().unwrap(),
                "pom.xml"
            )?;

            debug!("Module POM for '{}' downloaded successfully.", module);
        }
    }

    // Step 3: Generate and analyze the effective POM
    let pom_file_path = Path::new(output_folder).join("pom.xml");
    generate_and_analyze_effective_pom(config, output_folder, versions_keywords, &pom_file_path, repo_name)
}

