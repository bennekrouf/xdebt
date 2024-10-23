
use tracing::warn;

use crate::plugins::maven::process_pom::process_pom;
use crate::models::{AppConfig, Analysis, DependencyVersion};
use crate::plugins::maven::check_pom_xml_exists::check_pom_xml_exists;

pub async fn analyze_maven(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
    output_folder: &str,
    versions_keywords: &[&str],
    analyses: &mut Vec<Analysis>,
) {
    // Check for pom.xml in various possible locations
    match check_pom_xml_exists(config, project_name, repo_name).await {
        Ok(Some(pom_url)) => {
            // If a valid pom.xml is found, process it
            match process_pom(config, project_name, repo_name, output_folder, &pom_url, versions_keywords).await {
                Ok(versions_map) => {
                    analyses.extend(versions_map.iter().map(|(product, value)| Analysis {
                        repository_name: repo_name.to_string(),
                        dependency_version: DependencyVersion {
                            product: product.clone(),
                            cycle: value.as_str().unwrap_or("").to_string(),
                        },
                        roadmap: None,
                    }));
                }
                Err(e) => warn!("Failed to generate POM analysis for project '{}', repo '{}': {}", project_name, repo_name, e),
            }
        }
        Ok(None) => {
            warn!("No pom.xml found for project '{}', repo '{}'. Skipping.", project_name, repo_name);
        }
        Err(e) => warn!("Error while checking for pom.xml: {}", e),
    }
}

