
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use serde_json::{json, Value};
use std::path::Path;
use tracing::{info, error, debug};
use crate::models::AppConfig;
use crate::types::MyError;

pub fn append_json_to_file<'a>(
    config: &'a AppConfig,
    project_name: &str,
    json_data: &Value) -> Result<(), MyError> {
    // Get the target folder from the environment, default to "tmp" if not set
    let output_folder = &config.output_folder;
    let project_name = project_name.to_lowercase();

    // Ensure the target folder exists
    if !Path::new(&output_folder).exists() {
        info!("Target folder '{}' does not exist, creating it.", output_folder);
        create_dir_all(&output_folder)
            .map_err(|e| {
                error!("Error creating target folder '{}': {}", output_folder, e);
                format!("Failed to create target folder '{}': {}", output_folder, e)
            })?;
    }

    // Create the file path for the project JSON file
    let file_path = format!("{}/{}.json", &output_folder, project_name);
    debug!("Overwriting JSON data to file: {}", file_path);

    // Try to create or open the file, with truncation enabled
    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)  // Enable write mode
        .truncate(true)  // Truncate the file, i.e., overwrite its content
        .open(&file_path)
    {
        Ok(f) => f,
        Err(e) => {
            error!("Error opening file {}: {}", file_path, e);
            return Err(format!("Error opening file {}: {}", file_path, e).into());
        }
    };

    // Prepare the JSON data to write
    let json_entry = json!(json_data);
    debug!("Prepared JSON data for writing: {}", serde_json::to_string_pretty(&json_entry)?);

    // Write the JSON data to the file
    if let Err(e) = writeln!(file, "{}", serde_json::to_string_pretty(&json_entry)?) {
        error!("Error writing to file {}: {}", file_path, e);
        return Err(format!("Error writing to file {}: {}", file_path, e).into());
    }

    debug!("Successfully wrote JSON data to {}", file_path);

    Ok(())
}

