
use crate::types::MyError;

use crate::plugins::php::check_php_files::check_php_files;
use crate::models::{AppConfig, Analysis, DependencyVersion};

pub async fn check_php(
    config: &AppConfig,
    project_name: &str,
    repository_name: &str,
    repository_name_str: &str,
    analyses: &mut Vec<Analysis>,
) -> Result<(), MyError> {
    if check_php_files(config, project_name, repository_name).await? {
        analyses.push(Analysis {
            repository_name: repository_name_str.to_string(),
            dependency_version: DependencyVersion {
                product: "PHP".to_string(),
                cycle: "exists".to_string(),
            },
            roadmap: None,
        });
    }
    Ok(())
}
