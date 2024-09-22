use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::{HeaderName, HeaderValue, AUTHORIZATION, USER_AGENT};
use std::{env, error::Error};

/// Function to create an HTTP client and authorization header for Bitbucket or GitHub
pub fn create_client_with_auth(
    platform: String,
) -> Result<(Client, (HeaderName, HeaderValue), (HeaderName, HeaderValue)), Box<dyn Error>> {
    dotenv().ok(); // Load environment variables from .env file

    // Create the HTTP client
    let client = Client::new();
    let user_agent_header = HeaderValue::from_str("bennekrouf")?;

    // Handle authentication for Bitbucket
    if platform.to_lowercase() == "bitbucket" {
        let username = env::var("BITBUCKET_USERNAME")
            .map_err(|e| format!(
                "Missing BITBUCKET_USERNAME environment variable: {}. \
                Please create a `.env` file at the root of the repository and add a var BITBUCKET_USERNAME=<your_username>",
                e
            ))?;

        let password = env::var("BITBUCKET_PASSWORD")
            .map_err(|e| format!(
                "Missing BITBUCKET_PASSWORD environment variable: {}. \
                Please create a `.env` file at the root of the repository and add a var BITBUCKET_PASSWORD=<your_password>",
                e
            ))?;

        // Create the authorization header for Bitbucket
        let auth_header = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("{}:{}", username, password))
        );

        // Return the tuple with the client and the (HeaderName, HeaderValue)
        Ok((
            client,
            (AUTHORIZATION, HeaderValue::from_str(&auth_header)?),
            (USER_AGENT, user_agent_header),
        ))
    }
    // Handle authentication for GitHub
    else if platform.to_lowercase() == "github" {
        let github_token = env::var("GITHUB_TOKEN")
            .map_err(|e| format!(
                "Missing GITHUB_TOKEN environment variable: {}. \
                Please create a `.env` file at the root of the repository and add a var GITHUB_TOKEN=<your_token>",
                e
            ))?;

        // Create the authorization header for GitHub
        let auth_header = format!("Bearer {}", github_token);
        // Return the tuple with the client and the (HeaderName, HeaderValue)
        Ok((
            client,
            (AUTHORIZATION, HeaderValue::from_str(&auth_header)?),
            (USER_AGENT, user_agent_header),
        ))
    }
    // Unsupported platform
    else {
        Err(format!(
            "Unsupported platform: {}. Supported platforms are 'bitbucket' and 'github'",
            platform
        )
        .into())
    }
}
