
use crate::types::MyError;
use crate::utils::check_file_exists::check_file_exists;
use crate::models::AppConfig;

pub async fn check_dockerfile_exists(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<bool, MyError> {

    // List of Docker-related files to check
    let docker_files = ["Dockerfile", "docker-compose.yml", ".dockerignore"];

    for file in &docker_files {
        if check_file_exists(config, project_name, repo_name, file).await?.is_some() {
            return Ok(true);
        }
    }

    Ok(false)
}
