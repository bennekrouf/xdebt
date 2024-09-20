
use std::error::Error;
use std::env;
use serde_json::Value;
use tracing::{info, warn, debug};
use reqwest::header::HeaderValue;

use crate::plugins::npm::analyze_package_json_content::analyze_package_json_content;
use crate::plugins::maven::process_pom::process_pom;
use crate::plugins::docker::check_dockerfile_exists::check_dockerfile_exists;
use crate::plugins::dotnet::check_csproj_files::check_csproj_files;
use crate::plugins::php::check_php_files::check_php_files;
use crate::plugins::jenkins::check_jenkins_file_exists::check_jenkins_file_exists;
use crate::plugins::jenkins::extract_version_from_groovy::extract_version_from_groovy;
use crate::utils::enrich_versions_with_roadmap::enrich_versions_with_roadmap;
use crate::services::get_distinct_products::get_distinct_products;
use crate::create_config::AppConfig;
use crate::models::{Analysis, ProductVersion};

pub fn analyze_one_repo(
    config: &AppConfig,
    project_name: &str,
    repo_name: &str,
) -> Result<Vec<Analysis>, Box<dyn Error>> {
    let client = &config.client;
    let auth_header = &config.auth_header;
    let url_config = &*config.url_config;
    let db = &config.db;

    // Get the target folder from the environment
    let target_folder = env::var("TARGET_FOLDER")
        .unwrap_or_else(|_| "tmp".to_string());
    let target_folder = format!("{}/{}/{}", &target_folder, &project_name, &repo_name);

    // Fetch distinct product names from the Sled DB
    let product_names = get_distinct_products(db)?;

    // Convert Vec<String> to Vec<&str>
    let versions_keywords: Vec<&str> = product_names.iter().map(|s| s.as_str()).collect();

    // Get POM URL using UrlConfig
    let pom_url = url_config.file_url(project_name, repo_name, "pom.xml");

    // Try to process POM and continue even if there's an error
    let mut analyses = match process_pom(
        config, repo_name, &target_folder, &pom_url, &versions_keywords,
    ) {
        Ok(versions_map) => {
            // Convert the `Map<String, Value>` to `Vec<Analysis>`
            versions_map
                .iter()
                .map(|(product_name, value)| Analysis {
                    product_version: ProductVersion {
                        product_name: product_name.clone(),
                        version_number: value.as_str().unwrap_or("").to_string(),
                    },
                    roadmap: None, // Assuming `roadmap` is not present in the POM analysis result
                })
                .collect::<Vec<Analysis>>()
        }
        Err(e) => {
            warn!("Failed to generate POM analysis for project '{}', repo '{}': {}", project_name, repo_name, e);
            Vec::new()
        }
    };

    // Call the modified package.json analysis that now handles package.json search internally
    info!("Analyzing package.json for repository: {}", repo_name);
    let package_json_analysis_result = analyze_package_json_content(config, project_name, repo_name, &versions_keywords)?;

    // Convert package.json analysis result into `Analysis` structs and add them to `analyses`
    let package_json_analyses: Vec<Analysis> = package_json_analysis_result
        .get("analyses")
        .and_then(Value::as_array)
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|v| serde_json::from_value::<Analysis>(v.clone()).ok())
        .collect();

    analyses.extend(package_json_analyses);

    // Check if Dockerfile exists in the repository
    let dockerfile_exists = check_dockerfile_exists(config, project_name, repo_name)?;
    if dockerfile_exists {
        analyses.push(Analysis {
                    product_version: ProductVersion {
                        product_name: "Docker".to_string(),
                        version_number: "exists".to_string(),
                    },
                    roadmap: None,
        });
    }

    // Check if .csproj exists in the repository
    let csproj_exists = check_csproj_files(config, project_name, repo_name)?;
    if csproj_exists {
        analyses.push(Analysis {
                    product_version: ProductVersion {
                        product_name: "C#".to_string(),
                        version_number: "exists".to_string(),
                    },
                    roadmap: None,
        });
    }

    // Check if PHP repository files exist
    let php_files_exists = check_php_files(config, project_name, repo_name)?;
    if php_files_exists {
        analyses.push(Analysis {
                    product_version: ProductVersion {
                        product_name: "PHP".to_string(),
                        version_number: "exists".to_string(),
                    },
                    roadmap: None,
        });
    }

    // Jenkins analysis: Check if Jenkins file exists
    if let Some(jenkins_file_url) = check_jenkins_file_exists(config, project_name, repo_name)? {
        info!("Found Jenkins file at: {}", jenkins_file_url);
        let jenkins_file_content = client.get(&jenkins_file_url)
            .header("Authorization", HeaderValue::from_str(auth_header)?)
            .send()?
            .text()?;

        // Extract versions for each keyword from the Jenkins file
        for keyword in &versions_keywords {
            if let Some(version) = extract_version_from_groovy(config, &jenkins_file_content, keyword) {
                analyses.push(Analysis {
                    product_version: ProductVersion {
                        product_name: keyword.to_string(),
                        version_number: version.to_string(),
                    },
                    roadmap: None,
                });
            } else {
                info!("No version found for keyword '{}' in Jenkins file.", keyword);
            }
        }
    } else {
        info!("No Jenkins file found for project '{}', repo '{}'.", project_name, repo_name);
    }

    debug!("Final result of analysis for project '{}', repo '{}': {:?}", project_name, repo_name, analyses);

    // Enrich analyses with the roadmap data
    let enriched_analyses = enrich_versions_with_roadmap(db, analyses)?;

    Ok(enriched_analyses)
}

