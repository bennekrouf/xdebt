
use std::error::Error;
use crate::utils::check_file_exists::check_file_exists;
use crate::models::AppConfig;
use tracing::{debug, info};

pub async fn check_pom_xml_exists(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    // List of pom.xml locations to check
    let pom_xml_paths = [
        "pom.xml",
        "app/back/pom.xml",
    ];

    // Loop through the possible file paths
    for file in &pom_xml_paths {
        info!(
            "Checking for pom.xml at path: {} in repo: {}/{}",
            file, project_name, repo_name
        );

        match check_file_exists(config, project_name, repo_name, file).await {
            Ok(Some(file_url)) => {
                info!("Found pom.xml at: {}", file_url);
                return Ok(Some(file_url));  // Return the first valid pom.xml URL
            }
            Ok(None) => {
                debug!("No pom.xml found at path: {}", file);
            }
            Err(e) if e.to_string().contains("404") => {
                debug!("404 Not Found at path: {}. Continuing to the next path.", file);
            }
            Err(e) => {
                info!("Error occurred while checking for pom.xml at path: {}. Error: {}", file, e);
                return Err(e);
            }
        }
    }

    info!(
        "No pom.xml found for repo: {}/{} in any of the specified paths",
        project_name, repo_name
    );
    Ok(None)
}
