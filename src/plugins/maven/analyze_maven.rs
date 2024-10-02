
use tracing::warn;

use crate::plugins::maven::process_pom::process_pom;
use crate::models::{AppConfig, Analysis, DependencyVersion};

pub fn analyze_maven(
    config: &AppConfig,
    project_name: &str,
    repository_name: &str,
    output_folder: &str,
    versions_keywords: &[&str],
    analyses: &mut Vec<Analysis>,
) {
    let pom_url = config.url_config.raw_file_url(project_name, repository_name, "pom.xml");
    match process_pom(config, project_name, repository_name, output_folder, &pom_url, versions_keywords) {
        Ok(versions_map) => {
            analyses.extend(versions_map.iter().map(|(product, value)| Analysis {
                repository_name: repository_name.to_string(),
                dependency_version: DependencyVersion {
                    product: product.clone(),
                    cycle: value.as_str().unwrap_or("").to_string(),
                },
                roadmap: None,
            }));
        }
        Err(e) => warn!("Failed to generate POM analysis for project '{}', repo '{}': {}", project_name, repository_name, e),
    }
}
