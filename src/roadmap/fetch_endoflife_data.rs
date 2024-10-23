
// use reqwest::blocking::Client;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use tracing::{info, trace, debug};

// Fetch data from the End of Life API for a specific product
pub async fn fetch_endoflife_data(product: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    debug!("Fetching end-of-life data for product: {}", product);

    let url = format!("https://endoflife.date/api/{}.json", product);
    let client = Client::new();

    match client.get(&url).header("Accept", "application/json").send().await {
        Ok(response) => {
            trace!("Received response for product: {} content : {:?}", product, response);
            let json_data: Vec<Value> = response.json().await?;
            trace!("Parsed JSON data for product: {} content : {:?}", product, json_data);
            Ok(json_data)
        }
        Err(err) => {
            info!("Failed to fetch data from API for product {}: {:?}", product, err);
            // Err(Box::new(err))
            Ok(vec!())
        }
    }
}

