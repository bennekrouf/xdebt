
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::path::{Path, PathBuf};
use serde_json::{Value, Map};

use crate::utils::analyze_pom_content::analyze_pom_content;
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
    force_refresh: bool,
) -> Result<Map<String, Value>, Box<dyn Error>> {
    let pom_file_path: PathBuf = Path::new(target_folder).join(format!("{}_pom.xml", repo_name));

    // Check if the POM file already exists and handle FORCE_REFRESH
    if pom_file_path.exists() && !force_refresh {
        println!("POM file '{}' already exists, skipping download.", pom_file_path.display());
    } else {
        download_file(&client, &auth_header, &pom_url, &target_folder, repo_name)
            .map_err(|e| format!("Error while downloading POM file: {}", e))?;
    }

    let effective_pom_result = run_maven_effective_pom(&pom_file_path.to_string_lossy(), repo_name);

    let mut pom_versions = Map::new();
    if let Ok(effective_pom_file) = effective_pom_result {
        let effective_pom_path = Path::new(target_folder).join(&effective_pom_file);
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

    Ok(pom_versions)
}

