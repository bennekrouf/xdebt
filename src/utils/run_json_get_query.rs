use serde_json::Value;
use std::error::Error;
use tracing::{debug, error, trace};

use crate::models::AppConfig;

pub fn run_json_get_query(
    config: &AppConfig,
    paginated_repos_url: &str,
) -> Result<Value, Box<dyn Error>> {
    let client = &config.client;
    let headers = config.url_config.get_headers()?;

    // Perform the request
    let mut request = client
        .get(paginated_repos_url);

    for (name, value) in headers {
        request = request.header(name, value);
    }

    let response = request.send();

    match response {
        Ok(resp) => {
            // Trace the response status
            debug!("Received response with status: {}", resp.status());

            if resp.status().is_success() {
                let repos_body = resp.text();

                match repos_body {
                    Ok(body) => {
                        // Trace the body received
                        // debug!("Received body: {}", body);

                        let repos_json: Result<Value, _> = serde_json::from_str(&body);
                        match repos_json {
                            Ok(json) => {
                                // Trace successful JSON parsing
                                trace!("Successfully parsed JSON");
                                Ok(json)
                            }
                            Err(e) => {
                                trace!("Error parsing repos JSON: {}", e);
                                // error!("Raw body received: {}", body); // Show the problematic body
                                Err(format!("Error parsing repos JSON: {}", e).into())
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error reading repos response body: {}", e);
                        Err(format!("Error reading repos response body: {}", e).into())
                    }
                }
            } else {
                error!("Failed to fetch repos, status: {}", resp.status());
                Err(format!("Failed to fetch repos, status: {}", resp.status()).into())
            }
        }
        Err(e) => {
            error!("Error fetching repos URL {}: {}", paginated_repos_url, e);
            Err(format!("Error fetching repos URL {}: {}", paginated_repos_url, e).into())
        }
    }
}
