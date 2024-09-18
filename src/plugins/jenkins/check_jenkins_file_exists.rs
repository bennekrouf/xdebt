
use reqwest::blocking::Client;
use std::error::Error;
use crate::utils::check_file_exists::check_file_exists;

pub fn check_jenkins_file_exists(
    client: &Client,
    auth_header: &str,
    project_name: &str,
    repo_name: &str,
) -> Result<bool, Box<dyn Error>> {
    // List of Groovy files to check
    let groovy_files = [
        "Jenkinsfile.groovy", 
        "jenkins/devex/snap/Jenkinsfile.groovy",
        "jenkins/devex/stable/Jenkinsfile.groovy",
        "jenkins/snap.groovy",
        "jenkins/stable.groovy",
    ];

    for file in &groovy_files {
        if check_file_exists(client, auth_header, project_name, repo_name, file)? {
            return Ok(true);
        }
    }

    Ok(false)
}
