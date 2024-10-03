
use roxmltree::Document;
use std::collections::HashMap;
use std::error::Error;
use regex::Regex;
use tracing::{info, trace, debug};

pub fn extract_versions_from_doc(
    doc: &Document,
    equivalences: &HashMap<String, Vec<String>>,
    version_keywords: &[&str],
    version_regex: &Regex,
    versions: &mut HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    for keyword in version_keywords {
        info!("Analyzing keyword: '{}'", keyword);

        // 1. Handle <dependency> blocks with groupId, artifactId, and version
        for dep in doc.descendants().filter(|node| node.tag_name().name() == "dependency") {
            trace!("Analyzing dependency node: {:?}", dep);

            let group_id_node = dep.descendants().find(|node| node.tag_name().name() == "groupId");
            let artifact_id_node = dep.descendants().find(|node| node.tag_name().name() == "artifactId");
            let version_node = dep.descendants().find(|node| node.tag_name().name() == "version");

            if let (Some(group_id_node), Some(artifact_id_node), Some(version_node)) = (group_id_node, artifact_id_node, version_node) {
                let group_id_text = group_id_node.text().unwrap_or("");
                let artifact_id_text = artifact_id_node.text().unwrap_or("");
                let version_text = version_node.text().unwrap_or("");

                trace!("Found dependency: groupId='{}', artifactId='{}', version='{}'", group_id_text, artifact_id_text, version_text);

                // Check equivalences for the keyword
                for (equiv_keyword, references) in equivalences {
                    if equiv_keyword == keyword {
                        for reference in references {
                            if group_id_text.contains(reference) || artifact_id_text.contains(reference) {
                                debug!("Found matching reference '{}' for keyword '{}'", reference, keyword);

                                // Use regex to extract version if applicable
                                if let Some(caps) = version_regex.captures(version_text) {
                                    let extracted_version = caps.get(1).map(|m| m.as_str()).unwrap_or("unknown");
                                    versions.insert(keyword.to_string(), extracted_version.to_string());
                                    debug!("Extracted version '{}' for keyword '{}'", extracted_version, keyword);
                                } else {
                                    // Store the raw version text if no regex applies
                                    versions.insert(keyword.to_string(), version_text.to_string());
                                    debug!("Stored version '{}' for keyword '{}'", version_text, keyword);
                                }
                            }
                        }
                    }
                }

                // Direct match on artifactId
                if artifact_id_text == *keyword {
                    let cleaned_version = version_text.trim_start_matches('~').trim_start_matches('^');
                    versions.insert(keyword.to_string(), cleaned_version.to_string());
                    debug!("Directly matched and found version '{}' for artifactId '{}'", cleaned_version, artifact_id_text);
                }
            }
        }

        // 2. Handle <property> style tags like <devex-maven-plugin.version>
        let version_key = format!("{}.version", keyword); // Example: "devex-maven-plugin.version"
        trace!("Checking for property-style version string with key '{}'", version_key);

        // Look for nodes with the tag in the format of <keyword.version>
        for node in doc.descendants() {
            let tag_name = node.tag_name().name();
            if tag_name == version_key {
                if let Some(cycle) = node.text() {
                    let cleaned_version = cycle.trim_start_matches('~').trim_start_matches('^');
                    versions.insert(keyword.to_string(), cleaned_version.to_string());
                    info!("Found version '{}' for keyword '{}'", cleaned_version, keyword);
                }
            }
        }

        // 3. Handle <properties> section in POM where versions are defined like:
        //    <properties>
        //       <spring-boot.version>2.3.1.RELEASE</spring-boot.version>
        //    </properties>
        if let Some(properties_node) = doc.descendants().find(|node| node.tag_name().name() == "properties") {
            trace!("Analyzing <properties> section for '{}'", version_key);

            for prop in properties_node.descendants().filter(|node| node.tag_name().name() == &version_key) {
                if let Some(version_text) = prop.text() {
                    let cleaned_version = version_text.trim_start_matches('~').trim_start_matches('^');
                    versions.insert(keyword.to_string(), cleaned_version.to_string());
                    info!("Found version '{}' for keyword '{}' in properties", cleaned_version, keyword);
                }
            }
        }
    }

    Ok(())
}

