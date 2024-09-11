
use roxmltree::Document;
use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;
use std::error::Error;

pub fn analyze_pom_content(
    app_name: &str, 
    content: &str, 
    version_keywords: &[&str],
    reference_keywords: &[&str]
) -> Result<Value, Box<dyn Error>> {
    let mut versions = HashMap::new();
    let mut references = Vec::new();

    let cleaned_content = content.replace("?>\n<", "?>\n<").replace("?>\r\n<", "?>\n<");

    // Parse the XML content
    let doc = Document::parse(&cleaned_content)?;

    // Extract versions from dependencies and basic version strings
    for keyword in version_keywords {
        // println!("Checking for version of keyword: {}", keyword);

        // Check for dependencies
        for dep in doc.descendants().filter(|node| node.tag_name().name() == "dependency") {
            let group_id = dep.descendants().find(|node| node.tag_name().name() == "groupId");
            let artifact_id = dep.descendants().find(|node| node.tag_name().name() == "artifactId");

            if let (Some(group_id_node), Some(artifact_id_node)) = (group_id, artifact_id) {
                let group_id_text = group_id_node.text();
                let artifact_id_text = artifact_id_node.text();

                // Check if artifactId matches the keyword
                if artifact_id_text == Some(*keyword) {
                    // println!("Found dependency for artifactId: {}", keyword);

                    // Optionally, you could check for a specific groupId
                    // if let Some(group_id_value) = group_id_text {
                    //     println!("Found groupId: {}", group_id_value);
                    // }

                    if let Some(version_node) = dep.descendants().find(|node| node.tag_name().name() == "version") {
                        if let Some(version) = version_node.text() {
                            let cleaned_version = version.trim_start_matches('~').trim_start_matches('^');
                            println!("Version for {}: {}", keyword, cleaned_version);
                            versions.insert(keyword.to_string(), cleaned_version.to_string());
                        }
                    }
                }
            }
        }

        // Check for basic version strings in properties
        let version_key = format!("{}.version", keyword);
        if let Some(version_node) = doc.descendants().find(|node| node.tag_name().name() == &version_key) {
            if let Some(version) = version_node.text() {
                let cleaned_version = version.trim_start_matches('~').trim_start_matches('^');
                println!("Found version for {} in properties: {}", keyword, cleaned_version);
                versions.insert(keyword.to_string(), cleaned_version.to_string());
            }
        }
    }

    // Check for references in the content
    for &keyword in reference_keywords {
        // println!("Checking for presence of reference: {}", keyword);
        if content.contains(keyword) {
            println!("Found reference for: {}", keyword);
            references.push(keyword.to_string());
        }
    }

    // Build the JSON output
    let result = json!({
        "repository": app_name,
        "versions": versions,
        "references": references
    });

    println!("Final result: {}", result);

    Ok(result)
}

