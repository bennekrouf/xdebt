
// use sled;
use std::error::Error;
use std::fs;
use tracing::info;

use crate::roadmap::read_yaml::read_yaml;
use crate::roadmap::persist_to_sled::persist_to_sled;
use crate::models::AppConfig;

// Process all YAML files in the roadmap directory
pub fn process_yaml_files(config: &AppConfig, dir_path: &str) -> Result<(), Box<dyn Error>> {
    let db = config.db.as_ref().ok_or("Database is not initialized")?;
    // Iterate over each file in the directory
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and has a ".yml" extension
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yml") {
            if let Some(path_str) = path.to_str() {
                // Read and process the YAML file
                let roadmap_yaml = read_yaml(&config, path_str)?;
                persist_to_sled(db, &roadmap_yaml)?;
                info!("Processed file: {}", path_str); // Use trace for logging
            }
        }
    }
    Ok(())
}

