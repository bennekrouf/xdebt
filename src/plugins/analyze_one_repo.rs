
use std::error::Error;
use serde_json::Value;
use tracing::{info, warn, debug};

use crate::plugins::npm::analyze_package_json_content::analyze_package_json_content;
use crate::plugins::maven::process_pom::process_pom;
use crate::plugins::docker::check_dockerfile_exists::check_dockerfile_exists;
use crate::plugins::dotnet::check_csproj_files::check_csproj_files;
use crate::plugins::php::check_php_files::check_php_files;
use crate::plugins::jenkins::check_jenkins_file_exists::check_jenkins_file_exists;
use crate::plugins::jenkins::extract_version_from_groovy::extract_version_from_groovy;
use crate::utils::enrich_versions_with_roadmap::enrich_versions_with_roadmap;
use crate::services::get_distinct_dependencies::get_distinct_dependencies;
use crate::models::{AppConfig, Analysis, DependencyVersion};
use crate::utils::run_json_get_query::run_json_get_query;

pub fn analyze_one_repo<'a>(
    config: &'a AppConfig,
    project_name: &'a str,
    repository_name_str: &'a str,
) -> Result<Vec<Analysis>, Box<dyn Error>> {
    let repository_name = repository_name_str.to_string();
    let db = config.db.as_ref().expect("Db should be initialized");

    // Get the target folder from the environment
    let target_folder = format!("{}/{}/{}", config.output_folder, project_name, repository_name);

    // Fetch distinct dependency names from the Sled DB
    let dependency_names = get_distinct_dependencies(db)?;

    // Convert Vec<String> to Vec<&str>
    let versions_keywords: Vec<&str> = dependency_names.iter().map(|s| s.as_str()).collect();

    // Get POM URL using UrlConfig
    let pom_url = config.url_config.file_url(project_name, repository_name_str, "pom.xml");

    // Try to process POM and continue even if there's an error
    let mut analyses = match process_pom(
        config, repository_name_str, &target_folder, &pom_url, &versions_keywords,
    ) {
        Ok(versions_map) => versions_map.iter().map(|(dependency_name, value)| Analysis {
            repository_name: repository_name.clone(),
            dependency_version: DependencyVersion {
                dependency_name: dependency_name.clone(),
                version_number: value.as_str().unwrap_or("").to_string(),
            },
            roadmap: None,
        }).collect::<Vec<Analysis>>(),
        Err(e) => {
            warn!("Failed to generate POM analysis for project '{}', repo '{}': {}", project_name, repository_name, e);
            Vec::new()
        }
    };

    // Call the modified package.json analysis
    info!("Analyzing package.json for repository: {}", repository_name);
    let package_json_analysis_result = analyze_package_json_content(config, project_name, repository_name_str, &versions_keywords)?;

    // Convert package.json analysis result into `Analysis` structs
    let package_json_analyses: Vec<Analysis> = package_json_analysis_result
        .get("analyses")
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| serde_json::from_value::<Analysis>(v.clone()).ok())
        .collect();

    analyses.extend(package_json_analyses);

    // Check if Dockerfile exists in the repository
    if check_dockerfile_exists(config, project_name, repository_name_str)? {
        analyses.push(Analysis {
            repository_name: repository_name.clone(),
            dependency_version: DependencyVersion {
                dependency_name: "Docker".to_string(),
                version_number: "exists".to_string(),
            },
            roadmap: None,
        });
    }

    // Check if .csproj exists in the repository
    if check_csproj_files(config, project_name, repository_name_str)? {
        analyses.push(Analysis {
            repository_name: repository_name.clone(),
            dependency_version: DependencyVersion {
                dependency_name: "C#".to_string(),
                version_number: "exists".to_string(),
            },
            roadmap: None,
        });
    }

    // Check if PHP repository files exist
    if check_php_files(config, project_name, repository_name_str)? {
        analyses.push(Analysis {
            repository_name: repository_name.clone(),
            dependency_version: DependencyVersion {
                dependency_name: "PHP".to_string(),
                version_number: "exists".to_string(),
            },
            roadmap: None,
        });
    }

    // Jenkins analysis: Check if Jenkins file exists
    if let Some(jenkins_file_url) = check_jenkins_file_exists(config, project_name, repository_name_str)? {
        info!("Found Jenkins file at: {}", jenkins_file_url);

        // Use run_json_get_query to fetch the Jenkins file content
        let jenkins_file_content = run_json_get_query(config, &jenkins_file_url)?
            .to_string(); // Assuming the content is returned in a way that can be converted to string

        // Extract versions for each keyword from the Jenkins file
        for keyword in &versions_keywords {
            if let Some(version) = extract_version_from_groovy(config, &jenkins_file_content, keyword) {
                analyses.push(Analysis {
                    repository_name: repository_name.clone(),
                    dependency_version: DependencyVersion {
                        dependency_name: keyword.to_string(),
                        version_number: version.to_string(),
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

    debug!("Final result of analysis for project '{}', repo '{}': {:?}", project_name, repository_name, analyses);

    // Enrich analyses with the roadmap data
    let enriched_analyses = enrich_versions_with_roadmap(db, analyses)?;

    Ok(enriched_analyses)
}

