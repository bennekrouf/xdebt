
use roxmltree::Document;
use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use regex::Regex;

pub fn analyze_pom_content(
    app_name: &str, 
    content: &str, 
    version_keywords: &[&str],
    reference_keywords: &[&str]
) -> Result<Value, Box<dyn Error>> {
    // Define precise equivalences for version_keywords
    let mut equivalences: HashMap<&str, Vec<&str>> = HashMap::new();
    // equivalences.insert("spring", vec!["spring-context", "spring-beans", "spring-framework"]);
    // No need to include spring-boot in equivalences, let it be handled directly

    // Regex pattern to extract version numbers
    let version_regex = Regex::new(r"<version>([^<]+)</version>")?;

    let mut versions = HashMap::new();
    let mut references = Vec::new();

    let cleaned_content = content.replace("?>\n<", "?>\n<").replace("?>\r\n<", "?>\n<");

    // Parse the XML content
    let doc = Document::parse(&cleaned_content)?;

    // Initialize all version keywords with an empty string
    for keyword in version_keywords {
        versions.insert(keyword.to_string(), "".to_string());
    }

    // Extract versions from dependencies and basic version strings
    for keyword in version_keywords {
        // Check for dependencies
        for dep in doc.descendants().filter(|node| node.tag_name().name() == "dependency") {
            // Only check equivalences for keywords that are explicitly in the map
            if let Some(refs) = equivalences.get(keyword) {
                for &reference in refs {
                    // Search for the reference in the POM content
                    if content.contains(reference) {
                        // Extract the version number
                        if let Some(caps) = version_regex.captures(content) {
                            let version = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
                            versions.insert(keyword.to_string(), version.to_string());
                            break;  // Stop after the first occurrence
                        }
                    }
                }
            }

            let group_id = dep.descendants().find(|node| node.tag_name().name() == "groupId");
            let artifact_id = dep.descendants().find(|node| node.tag_name().name() == "artifactId");

            if let (Some(_group_id_node), Some(artifact_id_node)) = (group_id, artifact_id) {
                let artifact_id_text = artifact_id_node.text();

                // Check if artifactId matches the keyword directly
                if artifact_id_text == Some(*keyword) {
                    if let Some(version_node) = dep.descendants().find(|node| node.tag_name().name() == "version") {
                        if let Some(version) = version_node.text() {
                            let cleaned_version = version.trim_start_matches('~').trim_start_matches('^');
                            versions.insert(keyword.to_string(), cleaned_version.to_string());
                        }
                    }
                }
            }
        }

        // Check for basic version strings in properties, ignoring anything already matched by equivalences
        let version_key = format!("{}.version", keyword);
        if let Some(version_node) = doc.descendants().find(|node| node.tag_name().name() == &version_key) {
            if let Some(version) = version_node.text() {
                let cleaned_version = version.trim_start_matches('~').trim_start_matches('^');
                versions.insert(keyword.to_string(), cleaned_version.to_string());
            }
        }
    }

    // Check for references in the content
    for &keyword in reference_keywords {
        if content.contains(keyword) {
            references.push(keyword.to_string());
        }
    }

    // Build the JSON output
    let result = json!({
        "repository": app_name,
        "versions": versions,
        "references": references
    });

    Ok(result)
}


