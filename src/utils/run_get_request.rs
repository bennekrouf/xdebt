
use tracing::{error, trace, info};  // Import `trace`
use crate::models::AppConfig;
use crate::types::MyError;

pub fn run_get_request(
    config: &AppConfig,  // Use the config to get headers and client
    url: &str,           // URL to request
) -> Result<Option<String>, MyError> {  // Return `Option<String>`
    let client = &config.client;
    let headers = config.url_config.get_headers()?;  // Get headers from config

    // Trace the URL being requested
    trace!("Sending GET request to URL: {}", url);

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
            trace!("Received response with status: {}", resp.status());

            if resp.status().is_success() {
                // Return the raw response body as text
                match resp.text() {
                    Ok(body) => Ok(Some(body)),  // Wrap in `Some` for successful responses
                    Err(e) => {
                        info!("Error reading response body: {}", e);
                        Err(format!("Error reading response body: {}", e).into())
                    }
                }
            } else if resp.status().as_u16() == 404 {
                info!("Received 404 Not Found for URL: {}", url);
                Ok(None)  // Return `None` for 404
            } else {
                error!("Failed to fetch data from {}, status: {}", url, resp.status());
                Err(format!("Failed to fetch data, status: {}", resp.status()).into())
            }
        }
        Err(e) => {
            error!("Error fetching URL {}: {}", url, e);
            Err(format!("Error fetching URL {}: {}", url, e).into())
        }
    }
}

