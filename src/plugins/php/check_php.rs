
use std::error::Error;

use crate::plugins::php::check_php_files::check_php_files;
use crate::models::{AppConfig, Analysis, DependencyVersion};

pub fn check_php(
    config: &AppConfig,
    project_name: &str,
    repository_name: &str,
    repository_name_str: &str,
    analyses: &mut Vec<Analysis>,
) -> Result<(), Box<dyn Error>> {
    if check_php_files(config, project_name, repository_name)? {
        analyses.push(Analysis {
            repository_name: repository_name_str.to_string(),
            dependency_version: DependencyVersion {
                dependency_name: "PHP".to_string(),
                version_number: "exists".to_string(),
            },
            roadmap: None,
        });
    }
    Ok(())
}