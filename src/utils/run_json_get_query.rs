use crate::models::AppConfig;
use crate::utils::run_get_request::run_get_request;
use serde_json::Value;
use std::error::Error;
use tracing::{error, info, trace};

pub fn run_json_get_query(
    config: &AppConfig,
    paginated_repos_url: &str,
) -> Result<Value, Box<dyn Error>> {
    info!("URl fetched : {}", paginated_repos_url);
    // Use the reusable run_get_request function to get the raw body
    let body = run_get_request(config, paginated_repos_url)?;

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
            // error!("Raw body received: {}", body); // Show the problematic body for debugging
            Err(format!("Error parsing repos JSON: {}", e).into())
        }
    }
}
