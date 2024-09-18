
use std::error::Error;
use crate::utils::check_file_exists::check_file_exists;
use crate::create_config::AppConfig;

pub fn check_csproj_files(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<bool, Box<dyn Error>> {
    // List of .NET project files to check
    let csproj_files = ["*.csproj"]; // Adapt as needed

    for file in &csproj_files {
        if check_file_exists(config, project_name, repo_name, file)?.is_some() {
            return Ok(true);
        }
    }

    Ok(false)
}
