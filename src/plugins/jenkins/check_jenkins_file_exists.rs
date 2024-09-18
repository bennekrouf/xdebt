
use std::error::Error;
use crate::utils::check_file_exists::check_file_exists;
use crate::create_config::AppConfig;

pub fn check_jenkins_file_exists(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<Option<String>, Box<dyn Error>> {

    // List of Groovy files to check
    let groovy_files = [
        "Jenkinsfile.groovy", 
        "jenkins/devex/snap/Jenkinsfile.groovy",
        "jenkins/devex/stable/Jenkinsfile.groovy",
        "jenkins/snap.groovy",
        "jenkins/stable.groovy",
    ];

    for file in &groovy_files {
        if let Some(file_url) = check_file_exists(config, project_name, repo_name, file)? {
            return Ok(Some(file_url));
        }
    }

    Ok(None)
}
