
use std::error::Error;
use std::fs;
use crate::models::RoadmapList;

// Read and deserialize YAML from a file
pub fn read_yaml(file_path: &str) -> Result<RoadmapList, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;
    let roadmap: RoadmapList = serde_yaml::from_str(&file_content)?;
    Ok(roadmap)
}

