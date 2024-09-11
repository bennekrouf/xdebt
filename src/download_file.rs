
use reqwest::blocking::Client;
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;
use std::error::Error;

pub fn download_file(
    client: &Client,
    auth_header: &str,
    url: &str,
    target_folder: &str,
    repo_name: &str,
) -> Result<String, Box<dyn Error>> {
    // Ensure the target folder exists
    let target_path = Path::new(target_folder);
    if !target_path.exists() {
        fs::create_dir_all(target_folder)
            .map_err(|e| format!("Failed to create directory '{}': {}", target_folder, e))?;
    }

    let filename = format!("{}_pom.xml", repo_name);
    let full_path = target_path.join(&filename);

    // Perform the HTTP GET request with the authorization header
    let mut response = client
        .get(url)
        .header("Authorization", auth_header)
        .send()
        .map_err(|e| format!("Failed to send request to '{}': {}", url, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to download file: HTTP Status {} for URL '{}'",
            response.status(),
            url
        ).into());
    }

    // Create a file to save the content
    let mut file = File::create(&full_path)
        .map_err(|e| format!("Failed to create file '{}': {}", full_path.display(), e))?;
    
    // Write the content to the file
    copy(&mut response, &mut file)
        .map_err(|e| format!("Failed to write to file '{}': {}", full_path.display(), e))?;

    println!("File downloaded successfully to {:?}", full_path);

    Ok(full_path.to_string_lossy().to_string()) // Return the full path of the downloaded file
}

