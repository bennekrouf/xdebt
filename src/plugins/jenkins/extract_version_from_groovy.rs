
use std::collections::HashMap;
use tracing::info;
use crate::models::AppConfig;

pub fn extract_version_from_groovy(
    config: &AppConfig,
    properties: &HashMap<String, String>,
    keyword: &str,
) -> Option<String> {
    // Log the content of the properties
    info!("Jenkins properties:\n{:?}", properties);

    // Collect the keyword and its equivalences from the config
    let mut keywords_to_check = vec![keyword.to_string()]; // Start with the original keyword
    info!("Original keyword: {}", keyword);

    // Check if there are equivalences for the keyword
    if let Some(equivalences) = config.equivalences.get(keyword) {
        info!("Found equivalences for {}: {:?}", keyword, equivalences);
        keywords_to_check.extend(equivalences.clone()); // Add equivalences if they exist
    } else {
        info!("No equivalences found for keyword: {}", keyword);
    }

    // Iterate over each keyword and its equivalences, and try to find a version
    for kw in &keywords_to_check {
        info!("Checking keyword: {}", kw);

        // Attempt to find a version for the keyword in the properties
        if let Some(cycle) = properties.get(kw) {
            let version_str = cycle.trim(); // Optionally trim whitespace
            info!("Found version '{}' for keyword: {}", version_str, kw);
            return Some(version_str.to_string());
        } else {
            info!("No version found for keyword: {}", kw);
        }
    }

    info!("No version found for any keyword related to: {}", keyword);
    None
}

