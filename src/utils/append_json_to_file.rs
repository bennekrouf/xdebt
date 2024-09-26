
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use serde_json::Value;
use std::error::Error;
use serde_json::json;
use std::env;
use std::path::Path;
use tracing::{info, error, debug};

pub fn append_json_to_file(project_name: &str, json_data: &Value) -> Result<(), Box<dyn Error>> {
    // Get the target folder from the environment, default to "tmp" if not set
    let target_folder = env::var("TARGET_FOLDER")
        .unwrap_or_else(|_| {
            info!("TARGET_FOLDER not set, defaulting to 'tmp'");
            "tmp".to_string()
        });

    // Ensure the target folder exists
    if !Path::new(&target_folder).exists() {
        info!("Target folder '{}' does not exist, creating it.", target_folder);
        create_dir_all(&target_folder)
            .map_err(|e| {
                error!("Error creating target folder '{}': {}", target_folder, e);
                format!("Failed to create target folder '{}': {}", target_folder, e)
            })?;
    }

    // Create the file path for the project JSON file
    let file_path = format!("{}/{}.json", &target_folder, project_name);
    info!("Appending JSON data to file: {}", file_path);

    // Try to create or open the file
    let mut file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
    {
        Ok(f) => f,
        Err(e) => {
            error!("Error opening file {}: {}", file_path, e);
            return Err(format!("Error opening file {}: {}", file_path, e).into());
        }
    };

    // Prepare the JSON data to append
    let json_entry = json!(json_data);
    debug!("Prepared JSON data for writing: {}", serde_json::to_string_pretty(&json_entry)?);

    // Write the JSON data to the file
    if let Err(e) = writeln!(file, "{}", serde_json::to_string_pretty(&json_entry)?) {
        error!("Error writing to file {}: {}", file_path, e);
        return Err(format!("Error writing to file {}: {}", file_path, e).into());
    }

    info!("Successfully appended JSON data to {}", file_path);

    Ok(())
}

