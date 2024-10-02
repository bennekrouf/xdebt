
use std::error::Error;
use tracing::{info, debug};

use crate::plugins::jenkins::check_jenkins_file_exists::check_jenkins_file_exists;
use crate::plugins::jenkins::extract_version_from_groovy::extract_version_from_groovy;
use crate::models::{AppConfig, Analysis, DependencyVersion};
use crate::utils::run_get_request::run_get_request;
use crate::plugins::jenkins::parse_groovy_properties::parse_groovy_properties;

pub fn analyze_jenkins(
    config: &AppConfig,
    project_name: &str,
    repository_name: &str,
    versions_keywords: &[&str],
    repository_name_str: &str,
    analyses: &mut Vec<Analysis>,
) -> Result<(), Box<dyn Error>> {
    info!("Start of Jenkins analysis");

    if let Some(jenkins_file_url) = check_jenkins_file_exists(config, project_name, repository_name)? {
        info!("Found Jenkins file at: {}", jenkins_file_url);

        // Fetch the Jenkins file content
        let jenkins_file_content = run_get_request(config, &jenkins_file_url)?.unwrap_or_else(|| String::new());

        // Parse the Groovy properties
        let properties = parse_groovy_properties(&jenkins_file_content);

        for keyword in versions_keywords {
            // Try to extract the version from the parsed properties
            if let Some(cycle) = extract_version_from_groovy(config, &properties, keyword) {
                info!("Adding groovy props to analysis : {}/{}", keyword, cycle);
                analyses.push(Analysis {
                    repository_name: repository_name_str.to_string(),
                    dependency_version: DependencyVersion {
                        product: keyword.to_string(),
                        cycle: cycle.to_string(),
                    },
                    roadmap: None,
                });
            } else {
                info!("No version found for keyword '{}' in Jenkins file.", keyword);
            }
        }
    } else {
        info!("No Jenkins file found for project '{}', repo '{}'.", project_name, repository_name);
    }

    Ok(())
}

