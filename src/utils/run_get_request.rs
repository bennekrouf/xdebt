
use std::error::Error;
use tracing::{debug, error};
use crate::models::AppConfig;

pub fn run_get_request(
    config: &AppConfig,  // Use the config to get headers and client
    url: &str,           // URL to request
) -> Result<String, Box<dyn Error>> {
    let client = &config.client;
    let headers = config.url_config.get_headers()?;  // Get headers from config

    // Build the GET request
    let mut request = client.get(url);

    // Attach headers from config
    for (name, value) in headers {
        request = request.header(name, value);
    }

    // Send the request
    let response = request.send();

    // Process the response
    match response {
        Ok(resp) => {
            debug!("Received response with status: {}", resp.status());

            if resp.status().is_success() {
                // Return the raw response body as text
                match resp.text() {
                    Ok(body) => Ok(body),
                    Err(e) => {
                        error!("Error reading response body: {}", e);
                        Err(format!("Error reading response body: {}", e).into())
                    }
                }
            } else {
                error!("Failed to fetch data in {}, status: {}", url, resp.status());
                Err(format!("Failed to fetch data, status: {}", resp.status()).into())
            }
        }
        Err(e) => {
            error!("Error fetching URL {}: {}", url, e);
            Err(format!("Error fetching URL {}: {}", url, e).into())
        }
    }
}
