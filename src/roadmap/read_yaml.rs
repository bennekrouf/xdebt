
use std::error::Error;
use std::fs;
use crate::models::Versions;

// Read and deserialize YAML from a file
pub fn read_yaml(file_path: &str) -> Result<Versions, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let roadmap: Versions = serde_yaml::from_str(&file_content)?;
    Ok(roadmap)
}

