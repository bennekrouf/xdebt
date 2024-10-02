
use std::collections::HashMap;
use tracing::trace;

pub fn parse_groovy_properties(groovy_content: &str) -> HashMap<String, String> {
    let mut properties = HashMap::new();

    // Split the content by lines
    for line in groovy_content.lines() {
        // Trim whitespace and skip empty lines or comments
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
            continue;
        }

        // Look for key-value pairs in the format `key: 'value'`
        if let Some(pos) = trimmed_line.find(':') {
            let key = trimmed_line[..pos].trim().to_string();
            let value_part = &trimmed_line[pos + 1..];

            // Remove quotes and trim the value
            let value = value_part
                .trim()
                .trim_matches('\'')
                .trim()
                .to_string();

            properties.insert(key, value);
        }
    }

    trace!("Groovy properties : {:?}", properties);

    properties
}
