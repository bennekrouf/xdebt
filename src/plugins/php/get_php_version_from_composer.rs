
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use std::error::Error;

fn get_php_version_from_composer(repo_path: &str) -> Result<Option<String>, Box<dyn Error>> {
    let composer_file_path = format!("{}/composer.json", repo_path);
    let mut file = File::open(composer_file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let json: Value = serde_json::from_str(&content)?;
    if let Some(php_version) = json.get("require").and_then(|r| r.get("php")) {
        return Ok(Some(php_version.to_string()));
    }
    Ok(None)
}
