
use std::error::Error;
use tracing::{debug, error, info, trace}; // Add tracing macros
use roxmltree::Document;  // Or quick-xml depending on your preference

use crate::models::AppConfig;

pub fn run_json_get_query_xml(
    config: &AppConfig,
    file_url: &str,
) -> Result<Document<'static>, Box<dyn Error>> {
    let client = &config.client;
    let (auth_name, auth_value) = config.auth_header.clone(); // Extract auth header
    let (user_agent_name, user_agent_value) = config.auth_user_agent.clone();

    // Trace the URL we are going to fetch
    trace!("Fetching data from URL: {}", file_url);

    // Trace the headers that will be used in the request
    trace!("Using header: {} = {:?}", auth_name, auth_value);
    trace!(
        "Using User-Agent: {} = {:?}",
        user_agent_name,
        user_agent_value
    );

    // Perform the request
    let response = client
        .get(file_url)
        .header(auth_name, auth_value) // Use the extracted auth header
        // .header("User-Agent", user_agent_value) // Use the extracted user-agent value
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
                        // debug!("Received body: {}", body);

                        // Parse the body as XML using roxmltree or any XML parser
                        match Document::parse(&body) {
                            Ok(xml_doc) => {
                                // Trace successful XML parsing
                                trace!("Successfully parsed XML");
                                Ok(xml_doc)
                            }
                            Err(e) => {
                                trace!("Error parsing XML: {}", e);
                                // error!("Raw body received: {}", body); // Show the problematic body
                                Err(format!("Error parsing XML: {}", e).into())
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error reading repos response body: {}", e);
                        Err(format!("Error reading repos response body: {}", e).into())
                    }
                }
            } else {
                error!("Failed to fetch file, status: {}", resp.status());
                Err(format!("Failed to fetch file, status: {}", resp.status()).into())
            }
        }
        Err(e) => {
            error!("Error fetching file URL {}: {}", file_url, e);
            Err(format!("Error fetching file URL {}: {}", file_url, e).into())
        }
    }
}

