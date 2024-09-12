
use std::error::Error;
use std::env;
use base64::{engine::general_purpose, Engine as _};
use reqwest::blocking::Client;
use dotenv::dotenv;

/// Function to create an HTTP client and authorization header
pub fn create_client_with_auth() -> Result<(Client, String), Box<dyn Error>> {
    dotenv().ok();  // Load environment variables from .env file

    // Get credentials from environment variables
    let username = env::var("BITBUCKET_USERNAME")
        .map_err(|e| format!("Missing BITBUCKET_USERNAME environment variable: {}", e))?;
    let password = env::var("BITBUCKET_PASSWORD")
        .map_err(|e| format!("Missing BITBUCKET_PASSWORD environment variable: {}", e))?;

    // Create the HTTP client
    let client = Client::new();

    // Create an authorization header
    let auth_header = format!("Basic {}", general_purpose::STANDARD.encode(format!("{}:{}", username, password)));

    Ok((client, auth_header))
}

