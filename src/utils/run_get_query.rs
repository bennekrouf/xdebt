use serde_json::Value;
use std::error::Error;
use tracing::{debug, error, info, trace}; // Add tracing macros

use crate::models::AppConfig;

pub fn run_get_query(
    config: &AppConfig,
    paginated_repos_url: &str,
) -> Result<Value, Box<dyn Error>> {
    let client = &config.client;
    let (auth_name, auth_value) = config.auth_header.clone(); // Extract auth header
    let (user_agent_name, user_agent_value) = config.auth_user_agent.clone();

    // Trace the URL we are going to fetch
    info!("Fetching data from URL: {}", paginated_repos_url);

    // Trace the headers that will be used in the request
    trace!("Using header: {} = {:?}", auth_name, auth_value);
    trace!(
        "Using User-Agent: {} = {:?}",
        user_agent_name,
        user_agent_value
    );

    // Perform the request
    let response = client
        .get(paginated_repos_url)
        .header(auth_name, auth_value) // Use the extracted auth header
        .header("User-Agent", user_agent_value) // Use the extracted user-agent value
        .send();

    match response {
        Ok(resp) => {
            // Trace the response status
            debug!("Received response with status: {}", resp.status());

            if resp.status().is_success() {
                let repos_body = resp.text();

                match repos_body {
                    Ok(body) => {
                        // Trace the body received
                        debug!("Received body: {}", body);

                        let repos_json: Result<Value, _> = serde_json::from_str(&body);
                        match repos_json {
                            Ok(json) => {
                                // Trace successful JSON parsing
                                info!("Successfully parsed JSON");
                                Ok(json)
                            }
                            Err(e) => {
                                error!("Error parsing repos JSON: {}", e);
                                error!("Raw body received: {}", body); // Show the problematic body
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
