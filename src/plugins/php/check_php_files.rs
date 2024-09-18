
use reqwest::blocking::Client;
use std::error::Error;

use crate::utils::check_file_exists::check_file_exists;

pub fn check_php_files(
    client: &Client,
    auth_header: &str,
    project_name: &str,
    repo_name: &str,
) -> Result<bool, Box<dyn Error>> {
    // List of PHP-related files to check
    let php_files = [
        "composer.json", // Composer dependency file
        "php.ini",       // PHP configuration file
        "index.php",     // Common entry point
        "*.php"          // PHP source files
    ];

    for file in &php_files {
        if check_file_exists(client, auth_header, project_name, repo_name, file)?.is_some() {
            return Ok(true);
        }
    }

    Ok(false)
}
