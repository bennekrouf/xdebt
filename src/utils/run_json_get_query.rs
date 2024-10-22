
use serde_json::Value;

use tracing::{error, trace};
use crate::models::AppConfig;
use crate::utils::run_get_request::run_get_request;
use serde_json::json;  // For creating empty JSON objects

use crate::types::MyError;

pub fn run_json_get_query(
    config: &AppConfig,
    paginated_repos_url: &str,
) -> Result<Value, MyError> {
    // Use the reusable run_get_request function to get the raw body
    match run_get_request(config, paginated_repos_url)? {
        Some(body) => {
            // info!("run get request for json result : {}", body);
            // Parse the body as JSON
            let repos_json: Result<Value, _> = serde_json::from_str(&body);

            match repos_json {
                Ok(json) => {
                    // Trace successful JSON parsing
                    trace!("Successfully parsed JSON");
                    Ok(json)
                }
                Err(e) => {
                    error!("Error parsing repos JSON: {}", e);
                    Err(format!("Error parsing repos JSON: {}", e).into())
                }
            }
        }
        None => {
            // Log the lack of content but return an empty JSON object
            trace!("No content received from the URL: {}", paginated_repos_url);
            Ok(json!({}))  // Return an empty JSON object
        }
    }
}

