use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tracing::trace;

use crate::models::AppConfig;
use crate::utils::run_get_query::run_get_query;

pub fn download_file(
    config: &AppConfig,
    url: &str,
    target_folder: &str,
    file_name: &str,
) -> Result<String, Box<dyn Error>> {
    // Ensure the target folder exists
    trace!("Ensuring target folder '{}' exists.", target_folder);
    let target_path = Path::new(target_folder);
    if !target_path.exists() {
        trace!(
            "Target folder '{}' does not exist. Creating...",
            target_folder
        );
        fs::create_dir_all(target_folder)
            .map_err(|e| format!("Failed to create directory '{}': {}", target_folder, e))?;
    }

    let full_path = target_path.join(file_name);
    trace!("Full file path: {:?}", full_path);

    // Use run_get_query to perform the GET request
    trace!("Sending GET request to URL: {}", url);
    let response_json = run_get_query(config, url)?;

    // Check if the response indicates an error
    if response_json.get("error").is_some() {
        return Err(format!("Failed to download file: Error in response from '{}'", url).into());
    }

    // Assuming the file content is in a specific field of the JSON response, extract it
    // This part may need to be adjusted based on the actual structure of the response
    let file_content = response_json["content"]
        .as_str()
        .ok_or("Failed to get file content from response")?;

    // Create a file to save the content
    trace!("Creating file at '{}'.", full_path.display());
    let mut file = File::create(&full_path)
        .map_err(|e| format!("Failed to create file '{}': {}", full_path.display(), e))?;

    // Write the content to the file
    trace!("Writing content to file.");
    file.write_all(file_content.as_bytes())
        .map_err(|e| format!("Failed to write to file '{}': {}", full_path.display(), e))?;

    trace!("File downloaded successfully to {:?}", full_path);

    Ok(full_path.to_string_lossy().to_string())
}
