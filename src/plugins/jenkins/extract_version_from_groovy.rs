
use regex::Regex;

pub fn extract_version_from_groovy(content: &str, keyword: &str) -> Option<String> {
    // Regex pattern to match version assignments, e.g., "jdk : '17'"
    let pattern = format!(r"{}.*?:\s*'([^']+)'", keyword);
    let re = Regex::new(&pattern).unwrap();

    if let Some(captures) = re.captures(content) {
        return Some(captures[1].to_string());
    }

    None
}
