
use crate::types::MyError;
use tracing::trace;

pub fn parse_pom_for_modules(pom_content: &str) -> Result<Vec<String>, MyError> {
    trace!("Parsing POM content for modules");
    let doc = roxmltree::Document::parse(pom_content)
        .map_err(|e| format!("Failed to parse POM XML: {}", e))?;

    let mut modules = Vec::new();
    for node in doc.descendants() {
        if node.tag_name().name() == "module" {
            if let Some(module_name) = node.text() {
                modules.push(module_name.to_string());
            }
        }
    }
    Ok(modules)
}

