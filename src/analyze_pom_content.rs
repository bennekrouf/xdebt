
use roxmltree::Document;
use std::error::Error;

pub fn analyze_pom_content(content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut references = Vec::new();
    let cleaned_content = content.replace("?>\n<", "?>\n<").replace("?>\r\n<", "?>\n<");

    // Parse the XML content
    let doc = Document::parse(&cleaned_content)?;

    // Check for Java version
    if let Some(java_version_node) = doc.descendants().find(|node| node.tag_name().name() == "java.version") {
        if let Some(java_version) = java_version_node.text() {
            references.push(format!("Java (version {})", java_version));
        }
    }

    // Check for Spring Boot version
    if let Some(parent_node) = doc.descendants().find(|node| node.tag_name().name() == "parent") {
        if let Some(group_id_node) = parent_node.descendants().find(|node| node.tag_name().name() == "groupId") {
            if group_id_node.text() == Some("org.springframework.boot") {
                if let Some(version_node) = parent_node.descendants().find(|node| node.tag_name().name() == "version") {
                    if let Some(spring_boot_version) = version_node.text() {
                        references.push(format!("Spring Boot (version {})", spring_boot_version));
                    }
                }
            }
        }
    }

    // Check for Spring version
    if let Some(spring_version_node) = doc.descendants().find(|node| node.tag_name().name() == "spring.version") {
        if let Some(spring_version) = spring_version_node.text() {
            references.push(format!("Spring (version {})", spring_version));
        }
    } else if content.contains("spring") {
        references.push("Spring".to_string());
    }

    // Check for additional references: jencks, Hibernate, RichFaces, PHP
    if content.contains("php") {
        references.push("PHP".to_string());
    }

    // Check for Hibernate version in dependencies
    if let Some(hibernate_dep) = doc.descendants().find(|node| {
        node.tag_name().name() == "dependency"
            && node.descendants().any(|n| n.tag_name().name() == "artifactId" && n.text() == Some("hibernate-core"))
    }) {
        if let Some(version_node) = hibernate_dep.descendants().find(|node| node.tag_name().name() == "version") {
            if let Some(hibernate_version) = version_node.text() {
                references.push(format!("Hibernate (version {})", hibernate_version));
            }
        }
    }

    if content.contains("richfaces") {
        references.push("RichFaces".to_string());
    }
    if content.contains("jencks") {
        references.push("jencks".to_string());
    }
    if content.contains("nexus") {
        references.push("nexus".to_string());
    } else if content.contains("xfile") {
        references.push("xfile".to_string());
    }
    Ok(references)
}

