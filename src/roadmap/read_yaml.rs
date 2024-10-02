
use chrono::NaiveDate;
// use std::collections::HashMap;
use std::error::Error;
use std::fs;
use crate::models::{RoadmapList, RoadmapEntry};
use tracing::{info, error, trace};
use crate::roadmap::fetch_endoflife_data::fetch_endoflife_data;
use crate::models::AppConfig;

// Read and deserialize YAML from a file and enrich it with data from the End of Life API
pub fn read_yaml(config: &AppConfig, file_path: &str) -> Result<RoadmapList, Box<dyn Error>> {
    trace!("Reading YAML file from path: {}", file_path);

    // Read YAML content
    let file_content = fs::read_to_string(file_path)?;
    trace!("YAML content read successfully");

    let mut roadmap_list: RoadmapList = serde_yaml::from_str(&file_content)?;
    trace!("YAML content deserialized successfully");

    // Iterate over each product in the roadmap list and enrich it with End of Life data
    for roadmap in roadmap_list.roadmap_list.iter_mut() {
        let product = &roadmap.product;
        info!("Processing product: {}", product);

        // Fetch End of Life data for the product and its equivalences
        let product_names_to_check = get_product_and_equivalents(config, product);

        for product_name in product_names_to_check {
            trace!("Fetching End of Life data for product: {}", product_name);
            match fetch_endoflife_data(&product_name) {
                Ok(eol_data) => {
                    trace!("Fetched end-of-life data for product: {}", product_name);

                    // Create new roadmap entries from the API response
                    let eol_roadmap_entries: Vec<RoadmapEntry> = eol_data.iter().map(|entry| {
                        RoadmapEntry {
                            cycle: entry["cycle"].as_str().unwrap_or_default().to_string(),
                            release_date: entry["releaseDate"].as_str().map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()).flatten(),
                            eol: entry["eol"].as_str().map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()).flatten(),
                            extended_end_date: None, // API does not provide extended end date
                            comment: Some("Source: End-of-life API".to_string()),
                            source_name: Some("end-of-life".to_string()),
                            source_entity: Some("End-of-life API".to_string()),
                            updated_at: None,
                        }
                    }).collect();

                    // Add new roadmap entries to the existing roadmap for this product
                    roadmap.entries.extend(eol_roadmap_entries);
                }
                Err(err) => {
                    error!("Failed to fetch end-of-life data for product {}: {:?}", product_name, err);
                }
            }
        }
    }

    Ok(roadmap_list)
}

// Helper function to get product and its equivalent terms
fn get_product_and_equivalents(config: &AppConfig, product: &str) -> Vec<String> {
    let mut products_to_check = vec![product.to_string()];

    // Check if the product has equivalences in the config
    if let Some(equivalents) = config.equivalences.get(product) {
        products_to_check.extend(equivalents.iter().cloned());
    }

    products_to_check
}

