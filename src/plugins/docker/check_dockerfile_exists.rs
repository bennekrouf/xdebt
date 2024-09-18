
use reqwest::blocking::Client;
use std::error::Error;
use crate::utils::check_file_exists::check_file_exists;

pub fn check_dockerfile_exists(
    client: &Client,
    auth_header: &str,
    project_name: &str,
    repo_name: &str,
) -> Result<bool, Box<dyn Error>> {
    // List of Docker-related files to check
    let docker_files = ["Dockerfile", "docker-compose.yml", ".dockerignore"];

    for file in &docker_files {
        if check_file_exists(client, auth_header, project_name, repo_name, file)?.is_some() {
            return Ok(true);
        }
    }

    Ok(false)
}
