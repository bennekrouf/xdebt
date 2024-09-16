
use reqwest::blocking::Client;
use std::error::Error;
use crate::utils::check_file_exists::check_file_exists;

pub fn check_csproj_files(
    client: &Client,
    auth_header: &str,
    project_name: &str,
    repo_name: &str,
) -> Result<bool, Box<dyn Error>> {
    // List of .NET project files to check
    let csproj_files = ["*.csproj"]; // Adapt as needed

    for file in &csproj_files {
        if check_file_exists(client, auth_header, project_name, repo_name, file)? {
            return Ok(true);
        }
    }

    Ok(false)
}