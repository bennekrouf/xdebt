
use roxmltree::Document;
use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;
use crate::types::MyError;
use regex::Regex;
use tracing::{debug, info, trace};
use crate::models::AppConfig;
use crate::plugins::maven::utils::extract_versions_from_doc::extract_versions_from_doc;

pub fn analyze_pom_content(
    config: &AppConfig,
    app_name: &str, 
    content: &str, 
    version_keywords: &[&str],
) -> Result<Value, MyError> {
    let equivalences = config.equivalences.clone();
    info!("Analyzing POM content for app: '{}' and keywords : {:?}", app_name, version_keywords);

    let version_regex = Regex::new(r"<version>([^<]+)</version>")?;
    debug!("Initialized version extraction regex");

    let mut versions = HashMap::new();

    let cleaned_content = content.replace("?>\n<", "?>\n<").replace("?>\r\n<", "?>\n<");
    trace!("Cleaned content for parsing");

    let doc = Document::parse(&cleaned_content)?;
    debug!("XML document parsed successfully");

    // Call the new function to extract versions
    extract_versions_from_doc(&doc, &equivalences, version_keywords, &version_regex, &mut versions)?;

    let result = json!({
        "repository": app_name,
        "versions": versions,
    });

    info!("Finished analyzing POM content for '{}'", app_name);
    Ok(result)
}

