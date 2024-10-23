
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tracing::{debug, trace, error};
use roxmltree::Document;

use crate::utils::run_get_request::run_get_request;
use crate::models::AppConfig;
use crate::types::MyError;

pub async fn download_xml_file(
    config: &AppConfig,
    url: &str,
    output_folder: &str,
    file_name: &str,
) -> Result<String, MyError> {
    let output_folder = &output_folder.to_lowercase();

    // Ensure the target folder exists
    debug!("Ensuring target folder '{}' exists.", output_folder);
    let target_path = Path::new(&output_folder);

    if !target_path.exists() {
        debug!("Target folder '{}' does not exist. Creating...", output_folder);
        if let Err(e) = fs::create_dir_all(output_folder) {
            error!("Failed to create directory '{}': {}", output_folder, e);
            return Ok(String::new());  // Return empty string on failure
        }
    }

    let full_path = target_path.join(file_name);
    debug!("Full file path: {:?}", full_path);

    // Perform the GET request to retrieve the XML content
    debug!("Sending GET request to URL: {}", url);
    let body = match run_get_request(config, url).await? {
        Some(content) => content,
        None => {
            debug!("Failed to fetch XML from '{}'.", url);
            return Ok(String::new());  // Return empty string if no content
        }
    };

    // Parse the body as XML using roxmltree
    let file_content = match Document::parse(&body) {
        Ok(parsed) => parsed,
        Err(e) => {
            error!("Error parsing XML: {} related to folder : {}", e, &output_folder);
            return Ok(String::new());  // Return empty string if parsing fails
        }
    };

    // Create a file to save the content
    trace!("Creating file at '{}'.", full_path.display());
    let mut file = match File::create(&full_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to create file '{}': {}", full_path.display(), e);
            return Ok(String::new());  // Return empty string if file creation fails
        }
    };

    // Write the entire XML content to the file
    debug!("Writing XML content to file.");
    if let Err(e) = file.write_all(file_content.input_text().as_bytes()) {
        error!("Failed to write to file '{}': {}", full_path.display(), e);
        return Ok(String::new());  // Return empty string if writing to the file fails
    }

    debug!("File downloaded successfully to {:?}", full_path);

    Ok(full_path.to_string_lossy().to_string())  // Return the path as a string
}

