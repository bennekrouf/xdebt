
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::types::MyError;
use tracing::debug;

use crate::utils::download_xml_file::download_xml_file;
use crate::models::AppConfig;

/// Download and read the POM file, returning its content as a string.
pub async fn download_and_read_pom(
    config: &AppConfig,
    output_folder: &str,
    pom_url: &str,
    repo_name: &str,
) -> Result<String, MyError> {
    let pom_file_path: PathBuf = Path::new(output_folder).join("pom.xml");

    if pom_file_path.exists() && !config.force_git_pull {
        debug!("POM file '{}' already exists, skipping download.", pom_file_path.display());
    } else {
        debug!("Downloading POM file from '{}'", pom_url);
        let result = download_xml_file(config, pom_url, output_folder, "pom.xml").await;

        if let Err(e) = result {
            if e.to_string().contains("404 Not Found") {
                return Err(format!("POM file not found at URL '{}'. Skipping repository '{}'.", pom_url, repo_name).into());
            } else {
                return Err(format!("Error while downloading POM file: {}", e).into());
            }
        }
    }

    // Read the main POM file
    let mut main_pom_content = String::new();
    File::open(&pom_file_path)
        .and_then(|mut file| file.read_to_string(&mut main_pom_content))
        .map_err(|e| format!("Failed to read main POM file '{}': {}", pom_file_path.display(), e))?;

    Ok(main_pom_content)
}


