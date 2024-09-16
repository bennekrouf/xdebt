
use reqwest::blocking::Client;
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;
use std::error::Error;
use tracing::trace;

pub fn download_file(
    client: &Client,
    auth_header: &str,
    url: &str,
    target_folder: &str,
    file_name: &str,
) -> Result<String, Box<dyn Error>> {
    // Ensure the target folder exists
    trace!("Ensuring target folder '{}' exists.", target_folder);
    let target_path = Path::new(target_folder);
    if !target_path.exists() {
        trace!("Target folder '{}' does not exist. Creating...", target_folder);
        fs::create_dir_all(target_folder)
            .map_err(|e| format!("Failed to create directory '{}': {}", target_folder, e))?;
    }

    let full_path = target_path.join(file_name);
    trace!("Full file path: {:?}", full_path);

    // Perform the HTTP GET request with the authorization header
    trace!("Sending GET request to URL: {}", url);
    let mut response = client
        .get(url)
        .header("Authorization", auth_header)
        .send()
        .map_err(|e| format!("Failed to send request to '{}': {}", url, e))?;

    trace!("Response received with status: {}", response.status());
    if !response.status().is_success() {
        return Err(format!(
            "Failed to download file: HTTP Status {} for URL '{}'",
            response.status(),
            url
        ).into());
    }

    // Create a file to save the content
    trace!("Creating file at '{}'.", full_path.display());
    let mut file = File::create(&full_path)
        .map_err(|e| format!("Failed to create file '{}': {}", full_path.display(), e))?;

    // Write the content to the file
    trace!("Writing content to file.");
    copy(&mut response, &mut file)
        .map_err(|e| format!("Failed to write to file '{}': {}", full_path.display(), e))?;

    trace!("File downloaded successfully to {:?}", full_path);

    Ok(full_path.to_string_lossy().to_string())
}

