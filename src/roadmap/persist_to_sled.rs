
use serde_json;
use sled;
use tracing::trace;

use crate::models::Roadmaps;
use crate::types::MyError;

// Persist each product version to sled DB (case-insensitive)
pub fn persist_to_sled(db: &sled::Db, roadmap_yaml: &Roadmaps) -> Result<(), MyError> {
    for roadmap in &roadmap_yaml.roadmap_list {
        trace!("Persisting : {:?}", roadmap);
        let serialized_product = serde_json::to_vec(&roadmap)?;
        // Convert the product key to lowercase for case-insensitive persistence
        let dependency_lower = roadmap.product.to_lowercase();
        db.insert(dependency_lower.as_bytes(), serialized_product)?;
    }
    db.flush()?;
    Ok(())
}

