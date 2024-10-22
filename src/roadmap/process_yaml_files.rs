
use std::fs;
use tracing::info;

use crate::roadmap::read_yaml::read_yaml;
use crate::roadmap::persist_to_sled::persist_to_sled;
use crate::models::AppConfig;
use crate::types::MyError;

// Process all YAML files in the roadmap directory
pub fn process_yaml_files(config: &AppConfig, dir_path: &str) -> Result<(), MyError> {
    let db = config.db.as_ref().ok_or("Database is not initialized")?;

    // Check if the flag `force_sled_db_sourcing` is set to true
    if config.force_sled_db_sourcing {
        // If true, flush the database (clear all existing data)
        info!("Force sourcing enabled. Flushing the database.");
        db.clear()?;
    } else {
        // If not force sourcing, check if the database is empty
        if db.is_empty() {
            info!("Database is empty. Proceeding to load data.");
        } else {
            info!("Database is not empty. Skipping loading data.");
            return Ok(()); // Skip processing if the DB is not empty
        }
    }

    // Iterate over each file in the directory and process YAML files
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and has a ".yml" extension
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("yml") {
            if let Some(path_str) = path.to_str() {
                // Read and process the YAML file
                let roadmap_yaml = read_yaml(&config, path_str)?;
                persist_to_sled(db, &roadmap_yaml)?;
                info!("Processed file: {}", path_str);
            }
        }
    }

    Ok(())
}

