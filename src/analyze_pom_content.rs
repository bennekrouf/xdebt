
use roxmltree::Document;
use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;
use std::error::Error;

pub fn analyze_pom_content(
    app_name: &str, 
    content: &str, 
    reference_keywords: &[&str]
) -> Result<Value, Box<dyn Error>> {
    let mut versions = HashMap::new();
    let mut references = Vec::new();

    let cleaned_content = content.replace("?>\n<", "?>\n<").replace("?>\r\n<", "?>\n<");

    // Parse the XML content
    let doc = Document::parse(&cleaned_content)?;

    // Check for Java version
    if let Some(java_version_node) = doc.descendants().find(|node| node.tag_name().name() == "java.version") {
        if let Some(mut java_version) = java_version_node.text() {
            java_version = java_version.trim_start_matches('~').trim_start_matches('^');
            versions.insert("Java".to_string(), java_version.to_string());
        }
    }

    // Check for Spring Boot version
    if let Some(parent_node) = doc.descendants().find(|node| node.tag_name().name() == "parent") {
        if let Some(group_id_node) = parent_node.descendants().find(|node| node.tag_name().name() == "groupId") {
            if group_id_node.text() == Some("org.springframework.boot") {
                if let Some(version_node) = parent_node.descendants().find(|node| node.tag_name().name() == "version") {
                    if let Some(mut spring_boot_version) = version_node.text() {
                        spring_boot_version = spring_boot_version.trim_start_matches('~').trim_start_matches('^');
                        versions.insert("Spring Boot".to_string(), spring_boot_version.to_string());
                    }
                }
            }
        }
    }

    // Check for Spring version
    if let Some(spring_version_node) = doc.descendants().find(|node| node.tag_name().name() == "spring.version") {
        if let Some(mut spring_version) = spring_version_node.text() {
            spring_version = spring_version.trim_start_matches('~').trim_start_matches('^');
            versions.insert("Spring".to_string(), spring_version.to_string());
        }
    } else if content.contains("spring") {
        references.push("Spring".to_string());
    }

    // Dynamically check for all references passed in 'reference_keywords' array
    for &keyword in reference_keywords {
        if content.contains(keyword) {
            references.push(keyword.to_string());
        }
    }

    // Check for Hibernate version in dependencies
    if let Some(hibernate_dep) = doc.descendants().find(|node| {
        node.tag_name().name() == "dependency"
            && node.descendants().any(|n| n.tag_name().name() == "artifactId" && n.text() == Some("hibernate-core"))
    }) {
        if let Some(version_node) = hibernate_dep.descendants().find(|node| node.tag_name().name() == "version") {
            if let Some(mut hibernate_version) = version_node.text() {
                hibernate_version = hibernate_version.trim_start_matches('~').trim_start_matches('^');
                versions.insert("Hibernate".to_string(), hibernate_version.to_string());
            }
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

