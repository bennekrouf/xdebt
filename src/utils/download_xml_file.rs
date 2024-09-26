
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tracing::{debug, trace, error};
use roxmltree::Document;

use crate::utils::run_get_request::run_get_request;
use crate::models::AppConfig;

pub fn download_xml_file(
    config: &AppConfig,
    url: &str,
    target_folder: &str,
    file_name: &str,
) -> Result<String, Box<dyn Error>> {
    // Ensure the target folder exists
    debug!("Ensuring target folder '{}' exists.", target_folder);
    let target_path = Path::new(target_folder);
    if !target_path.exists() {
        debug!("Target folder '{}' does not exist. Creating...", target_folder);
        fs::create_dir_all(target_folder)
            .map_err(|e| format!("Failed to create directory '{}': {}", target_folder, e))?;
    }

    let full_path = target_path.join(file_name);
    debug!("Full file path: {:?}", full_path);

    // Perform the GET request to retrieve the XML content
    debug!("Sending GET request to URL: {}", url);
    let body = run_get_request(config, url)?;

    // Parse the body as XML using roxmltree
    let file_content = Document::parse(&body).map_err(|e| {
        error!("Error parsing XML: {}", e);
        format!("Error parsing XML: {}", e)
    })?;

    // Create a file to save the content
    trace!("Creating file at '{}'.", full_path.display());
    let mut file = File::create(&full_path)
        .map_err(|e| format!("Failed to create file '{}': {}", full_path.display(), e))?;

    // Write the entire XML content to the file
    debug!("Writing XML content to file.");
    file.write_all(file_content.input_text().as_bytes())
        .map_err(|e| format!("Failed to write to file '{}': {}", full_path.display(), e))?;

    debug!("File downloaded successfully to {:?}", full_path);

    Ok(full_path.to_string_lossy().to_string())
}

