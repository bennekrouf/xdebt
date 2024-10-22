
use crate::utils::check_file_exists::check_file_exists;
use crate::models::AppConfig;
use tracing::info;
use crate::types::MyError;

pub fn check_jenkins_file_exists(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<Option<String>, MyError> {
    // List of Groovy files to check
    let groovy_files = [
        "Jenkinsfile.groovy", 
        "jenkins/devex/snap/Jenkinsfile.groovy",
        "jenkins/devex/stable/Jenkinsfile.groovy",
        "jenkins/snap.groovy",
        "jenkins/stable.groovy",
    ];

    for file in &groovy_files {
        info!("Checking : {}", file);
        if let Some(file_url) = check_file_exists(config, project_name, repo_name, file)? {
            info!("content of jenkins file : {}", file_url);
            return Ok(Some(file_url));
        } else {
            info!("nothing returned from check jenkins file");
        }
    }

    Ok(None)
}
