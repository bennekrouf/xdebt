
use std::error::Error;

use crate::plugins::docker::check_dockerfile_exists::check_dockerfile_exists;
use crate::models::{AppConfig, Analysis, DependencyVersion};

pub fn check_docker(
    config: &AppConfig,
    project_name: &str,
    repository_name: &str,
    repository_name_str: &str,
    analyses: &mut Vec<Analysis>,
) -> Result<(), Box<dyn Error>> {
    if check_dockerfile_exists(config, project_name, repository_name)? {
        analyses.push(Analysis {
            repository_name: repository_name_str.to_string(),
            dependency_version: DependencyVersion {
                product: "Docker".to_string(),
                cycle: "exists".to_string(),
            },
            roadmap: None,
        });
    }
    Ok(())
}
