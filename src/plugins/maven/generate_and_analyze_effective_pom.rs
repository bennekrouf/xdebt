
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::{Value, Map};
use std::error::Error;
use tracing::{info, debug};

use crate::models::AppConfig;
use crate::plugins::maven::run_maven_effective_pom::run_maven_effective_pom;
use crate::plugins::maven::analyze_pom_content::analyze_pom_content;

pub fn generate_and_analyze_effective_pom(
    config: &AppConfig,
    output_folder: &str,
    versions_keywords: &[&str],
    pom_file_path: &Path,
    repo_name: &str,
) -> Result<Map<String, Value>, Box<dyn Error>> {
    let effective_pom_file = Path::new(output_folder).join("effective_pom.xml");
    let mut pom_versions = Map::new();

    if !effective_pom_file.exists() || config.force_maven_effective {
        info!(
            "Effective POM file '{}' does not exist or force_effective is true, generating effective POM.",
            effective_pom_file.display()
        );

        let effective_pom_result = run_maven_effective_pom(&pom_file_path.to_string_lossy())?;
        let effective_pom_path = Path::new(&effective_pom_result);

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
