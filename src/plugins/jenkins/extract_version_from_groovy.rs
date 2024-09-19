
use regex::Regex;
use tracing::{info, trace};
use crate::create_config::AppConfig;

pub fn extract_version_from_groovy(
    config: &AppConfig,
    content: &str,
    keyword: &str
) -> Option<String> {
    // Log the content of the Jenkins file
    trace!("Jenkins file content:\n{}", content);

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

    // Iterate over each keyword and its equivalences, and try to match a version
    for kw in &keywords_to_check {
        info!("Checking keyword: {}", kw);

        // Regex pattern to match version assignments, e.g., "jdk : '11'"
        let pattern = format!(r"{}.*?:\s*'([^']+)'", kw);
        trace!("Regex pattern being checked: {}", pattern);
        
        let re = Regex::new(&pattern).unwrap();

        // Attempt to find a version for the keyword
        if let Some(captures) = re.captures(content) {
            let version = captures[1].to_string();
            info!("Found version '{}' for keyword: {}", version, kw);
            return Some(version);
        } else {
            info!("No version found for keyword: {}", kw);
        }
    }

    info!("No version found for any keyword related to: {}", keyword);
    None
}

