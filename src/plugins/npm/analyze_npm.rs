
use crate::types::MyError;
use serde_json::Value;
use tracing::info;

use crate::plugins::npm::analyze_package_json_content::analyze_package_json_content;
use crate::models::{AppConfig, Analysis, DependencyVersion};

pub fn analyze_npm(
    config: &AppConfig,
    project_name: &str,
    repository_name: &str,
    versions_keywords: &[&str],
    analyses: &mut Vec<Analysis>,
) -> Result<(), MyError> {
    info!("Analyzing package.json for repository: {}", repository_name);

    // Analyze the package.json content
    let package_json_analysis_result = analyze_package_json_content(config, project_name, repository_name, versions_keywords)?;

    info!("package_json_analysis_result : {:?}", package_json_analysis_result);

    // Extract versions from the result
    let versions = package_json_analysis_result.get("versions").and_then(Value::as_object);

    if let Some(versions) = versions {
        // Map the versions into Analysis objects
        for (product, version_value) in versions {
            let version_str = version_value.as_str().unwrap_or_default();

            // Construct DependencyVersion
            let dependency_version = DependencyVersion {
                product: product.to_string(),
                cycle: version_str.to_string(),
            };

            // Create the Analysis object
            let analysis = Analysis {
                repository_name: repository_name.to_string(),
                dependency_version,
                roadmap: None, // Set this to None unless there's logic for it
            };

            info!(
                "Created analysis for product: {}, cycle: {}",
                product, version_str
            );
            analyses.push(analysis);
        }
    } else {
        info!("No versions found in the package.json result.");
    }

    Ok(())
}

