
use std::error::Error;
use crate::utils::check_file_exists::check_file_exists;
use crate::models::AppConfig;
use tracing::info;

pub fn check_package_json_exists(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    // List of package.json locations to check
    let package_json_paths = [
        "package.json",
        "front/package.json",
        "ui/package.json",  // add more paths as necessary
    ];

    // Loop through the possible file paths
    for file in &package_json_paths {
        // Log the current request being checked
        info!(
            "Checking for package.json at path: {} in repo: {}/{}",
            file, project_name, repo_name
        );

        // Call the function to check if the file exists
        match check_file_exists(config, project_name, repo_name, file) {
            Ok(Some(file_url)) => {
                info!("Found package.json at: {}", file_url);
                return Ok(Some(file_url));  // Return the first valid package.json URL
            }
            Ok(None) => {
                info!("No package.json found at path: {}", file);
            }
            Err(e) if e.to_string().contains("404") => {
                // Log the 404 error and continue to the next path
                info!("404 Not Found at path: {}. Continuing to the next path.", file);
            }
            Err(e) => {
                // For other errors, return them
                info!("Error occurred while checking for package.json at path: {}. Error: {}", file, e);
                return Err(e);
            }        }
    }

    // If no package.json is found, log the final result and return None
    info!(
        "No package.json found for repo: {}/{} in any of the specified paths",
        project_name, repo_name
    );
    Ok(None)
}

