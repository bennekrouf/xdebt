
use std::error::Error;
use tracing::{info, debug};

use crate::plugins::jenkins::check_jenkins_file_exists::check_jenkins_file_exists;
use crate::plugins::jenkins::extract_version_from_groovy::extract_version_from_groovy;
use crate::models::{AppConfig, Analysis, DependencyVersion};
use crate::utils::run_json_get_query::run_json_get_query;

pub fn analyze_jenkins(
    config: &AppConfig,
    project_name: &str,
    repository_name: &str,
    versions_keywords: &[&str],
    repository_name_str: &str,
    analyses: &mut Vec<Analysis>,
) -> Result<(), Box<dyn Error>> {
    info!("Start of jenkins analysis");
    if let Some(jenkins_file_url) = check_jenkins_file_exists(config, project_name, repository_name)? {
        info!("Found Jenkins file at: {}", jenkins_file_url);

        let jenkins_file_content = run_json_get_query(config, &jenkins_file_url)?
            .to_string(); 

        for keyword in versions_keywords {
            if let Some(version) = extract_version_from_groovy(config, &jenkins_file_content, keyword) {
                analyses.push(Analysis {
                    repository_name: repository_name_str.to_string(),
                    dependency_version: DependencyVersion {
                        dependency_name: keyword.to_string(),
                        current_version: version.to_string(),
                    },
                    roadmap: None,
                });
            } else {
                debug!("No version found for keyword '{}' in Jenkins file.", keyword);
            }
        }
    } else {
        info!("No Jenkins file found for project '{}', repo '{}'.", project_name, repository_name);
    }
    Ok(())
}

