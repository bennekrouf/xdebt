
use std::error::Error;
use crate::utils::check_file_exists::check_file_exists;
use crate::models::AppConfig;

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

    for file in &package_json_paths {
        if let Some(file_url) = check_file_exists(config, project_name, repo_name, file)? {
            return Ok(Some(file_url));  // Return the first valid package.json URL
        }
    }

    Ok(None)  // If no package.json is found
}
