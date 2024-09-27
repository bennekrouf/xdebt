
use roxmltree::Document;
use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use regex::Regex;
use tracing::{info, trace};
use crate::models::AppConfig;

pub fn analyze_pom_content(
    config: &AppConfig,
    app_name: &str, 
    content: &str, 
    version_keywords: &[&str],
) -> Result<Value, Box<dyn Error>> {
    let equivalences = config.equivalences.clone();
    info!("Analyzing POM content for app: '{}'", app_name);

    let version_regex = Regex::new(r"<version>([^<]+)</version>")?;
    info!("Initialized version extraction regex");

    let mut versions = HashMap::new();

    let cleaned_content = content.replace("?>\n<", "?>\n<").replace("?>\r\n<", "?>\n<");
    trace!("Cleaned content for parsing");

    let doc = Document::parse(&cleaned_content)?;
    info!("XML document parsed successfully");

    for keyword in version_keywords {
        info!("Analyzing keyword: '{}'", keyword);

        // Check for dependencies
        for dep in doc.descendants().filter(|node| node.tag_name().name() == "dependency") {
            trace!("Analyzing dependency node");

            // Check equivalences for keywords
            for (equiv_keyword, references) in &equivalences {
                if equiv_keyword == keyword {
                    for reference in references {
                        trace!("Checking equivalence reference '{}' for keyword '{}'", reference, keyword);

                        if content.contains(&*reference) {
                            info!("Found reference '{}' in content", reference);

                            // Extract the version number
                            if let Some(caps) = version_regex.captures(content) {
                                let version = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
                                versions.insert(keyword.to_string(), version.to_string());
                                info!("Found version '{}' for keyword '{}'", version, keyword);
                            }
                        }
                    }
                }
            }

            let group_id = dep.descendants().find(|node| node.tag_name().name() == "groupId");
            let artifact_id = dep.descendants().find(|node| node.tag_name().name() == "artifactId");

            if let (Some(_group_id_node), Some(artifact_id_node)) = (group_id, artifact_id) {
                let artifact_id_text = artifact_id_node.text();
                trace!("Found artifactId: {:?}", artifact_id_text);

                if artifact_id_text == Some(*keyword) {
                    info!("Found matching artifactId '{}' for keyword '{}'", artifact_id_text.unwrap(), keyword);

                    if let Some(version_node) = dep.descendants().find(|node| node.tag_name().name() == "version") {
                        if let Some(version) = version_node.text() {
                            let cleaned_version = version.trim_start_matches('~').trim_start_matches('^');
                            versions.insert(keyword.to_string(), cleaned_version.to_string());
                            info!("Found version '{}' for artifactId '{}'", cleaned_version, artifact_id_text.unwrap());
                        }
                    }
                }
            }
        }

        let version_key = format!("{}.version", keyword);
        trace!("Checking for basic version string in properties with key '{}'", version_key);

        if let Some(version_node) = doc.descendants().find(|node| node.tag_name().name() == &version_key) {
            if let Some(version) = version_node.text() {
                let cleaned_version = version.trim_start_matches('~').trim_start_matches('^');
                versions.insert(keyword.to_string(), cleaned_version.to_string());
                info!("Found version '{}' for keyword '{}'", cleaned_version, keyword);
            }
        }
    }

    let result = json!({
        "repository": app_name,
        "versions": versions,
    });

    info!("Finished analyzing POM content for '{}'", app_name);
    Ok(result)
}

