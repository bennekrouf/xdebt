
use serde_json::{Value, Map, json};
use sled::Db;
use std::error::Error;
use crate::services::get_product_version::get_product_version;

pub fn enrich_versions_with_roadmap(
    db: &Db,
    versions: &Map<String, Value>
) -> Result<Map<String, Value>, Box<dyn Error>> {
    let mut transformed_versions = Map::new();

    for (product, version_value) in versions.iter() {
        // Extract the current version as a string
        let current_version = version_value.as_str().unwrap_or("").to_string();

        // Get the product version (roadmap) from the sled DB
        let roadmap = match get_product_version(db, product)? {
            Some(roadmap) => {
                // Serialize the product version into JSON
                serde_json::to_value(roadmap)?
            },
            None => {
                // If no roadmap is found, set to null or some default
                Value::Null
            }
        };

        // Create the new version structure with "current" and "roadmap"
        let version_info = json!({
            "current": current_version,
            "roadmap": roadmap
        });

        // Insert into the transformed map
        transformed_versions.insert(product.clone(), version_info);
    }

    Ok(transformed_versions)
}
