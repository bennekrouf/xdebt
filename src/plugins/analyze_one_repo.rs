
use std::error::Error;
use tracing::debug;

use crate::utils::enrich_versions_with_roadmap::enrich_versions_with_roadmap;
use crate::services::get_distinct_dependencies::get_distinct_dependencies;
use crate::models::{AppConfig, Analysis};

use crate::plugins::php::check_php::check_php;
use crate::plugins::maven::analyze_maven::analyze_maven;
use crate::plugins::npm::analyze_npm::analyze_npm;
use crate::plugins::docker::check_docker::check_docker;
use crate::plugins::dotnet::check_dotnet::check_dotnet;
use crate::plugins::jenkins::analyze_jenkins::analyze_jenkins;

pub fn analyze_one_repo<'a>(
    config: &'a AppConfig,
    project_name: &'a str,
    repository_name_str: &'a str,
) -> Result<Vec<Analysis>, Box<dyn Error>> {
    let repository_name = repository_name_str.to_string();
    let db = config.db.as_ref().expect("Db should be initialized");

    let output_folder = format!("{}/{}/{}", config.output_folder, project_name, repository_name);
    let output_folder = output_folder.to_lowercase();

    let dependency_names = get_distinct_dependencies(db)?;
    let versions_keywords: Vec<&str> = dependency_names.iter().map(|s| s.as_str()).collect();

    let mut analyses = Vec::new();

    // 1. Maven (POM) Analysis
    if config.enable_maven_analysis {
        analyze_maven(config, project_name, repository_name_str, &output_folder, &versions_keywords, &mut analyses);
    }

    // 2. NPM (package.json) Analysis
    if config.enable_npm_analysis {
        analyze_npm(config, project_name, repository_name_str, &versions_keywords, &mut analyses)?;
    }

    // 3. Dockerfile Check
    if config.enable_docker_analysis {
        check_docker(config, project_name, repository_name_str, &repository_name, &mut analyses)?;
    }

    // 4. C# (.csproj) Analysis
    if config.enable_dotnet_analysis {
        check_dotnet(config, project_name, repository_name_str, &repository_name, &mut analyses)?;
    }

    // 5. PHP File Check
    if config.enable_php_analysis {
        check_php(config, project_name, repository_name_str, &repository_name, &mut analyses)?;
    }

    // 6. Jenkins File Analysis
    if config.enable_jenkins_analysis {
        analyze_jenkins(config, project_name, repository_name_str, &versions_keywords, &repository_name, &mut analyses)?;
    }

    debug!("Final result of analysis for project '{}', repo '{}': {:?}", project_name, repository_name, analyses);

    let enriched_analyses = enrich_versions_with_roadmap(db, analyses)?;
    Ok(enriched_analyses)
}
