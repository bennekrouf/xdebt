
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::{Value, Map};
use std::error::Error;
use std::env;
use tracing::{info, debug};

use crate::models::AppConfig;
use crate::plugins::maven::utils::generate_maven_effective_pom::generate_maven_effective_pom;
use crate::plugins::maven::utils::analyze_pom_content::analyze_pom_content;

pub fn generate_and_analyze_effective_pom(
    config: &AppConfig,
    versions_keywords: &[&str],
    pom_file_path: &Path,
    repo_name: &str,
    output_folder: &str,
) -> Result<Map<String, Value>, Box<dyn Error>> {
    // Effective POM file path (relative)
    let effective_pom_file = Path::new(output_folder).join("effective_pom.xml");

    // Log the current working directory
    let current_dir = env::current_dir()?;
    info!("Current working directory: {}", current_dir.display());

    // Log the absolute path of the effective POM file
    let absolute_effective_pom_file = current_dir.join(&effective_pom_file);
    info!("Effective POM file absolute path: {}", absolute_effective_pom_file.display());

    let mut pom_versions = Map::new();

    if !effective_pom_file.exists() || config.force_maven_effective {
        info!(
            "2 - Effective POM file '{}' does not exist or force_effective is true, generating effective POM.",
            effective_pom_file.display()
        );

        let _ = generate_maven_effective_pom(&pom_file_path.to_string_lossy())?;
        if !absolute_effective_pom_file.exists() {
            return Err(format!("1 - Effective POM file '{}' does not exist.", absolute_effective_pom_file.display()).into());
        }

        let mut content = String::new();
        File::open(&absolute_effective_pom_file)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| format!("Failed to read effective POM file '{}': {}", absolute_effective_pom_file.display(), e))?;

        // Analyze the POM content
        let pom_analysis_result = analyze_pom_content(config, repo_name, &content, versions_keywords)?;
        debug!("analyze_pom_content returns {}", pom_analysis_result);

        pom_versions.extend(pom_analysis_result.get("versions").and_then(Value::as_object).unwrap_or(&Map::new()).clone());

        return Ok(pom_versions);
    } else {
        info!(
            "3 - Effective POM file '{}' already exists, skipping generation.",
            effective_pom_file.display()
        );
    }

    Ok(pom_versions)
}

