
use serde_json;
use sled;
use std::error::Error;

use crate::models::RoadmapList;

// Persist each product version to sled DB (case-insensitive)
pub fn persist_to_sled(db: &sled::Db, roadmap_yaml: &RoadmapList) -> Result<(), Box<dyn Error>> {
    for roadmap in &roadmap_yaml.roadmap_list {
        let serialized_product = serde_json::to_vec(&roadmap)?;
        // Convert the product key to lowercase for case-insensitive persistence
        let product_lower = roadmap.product.to_lowercase();
        db.insert(product_lower.as_bytes(), serialized_product)?;
    }
    db.flush()?;
    Ok(())
}

