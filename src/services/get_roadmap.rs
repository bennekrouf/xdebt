
use serde_json;
use sled;
use tracing::trace;
use crate::models::Roadmap;
use crate::types::MyError;

// Get a product version from sled DB (case-insensitive)
pub fn get_roadmap(db: &sled::Db, product: &str) -> Result<Option<Roadmap>, MyError> {
    // Convert the product key to lowercase for case-insensitive search
    let dependency_lower = product.to_lowercase();

    if let Some(serialized_product) = db.get(dependency_lower.as_bytes())? {
        let roadmap: Roadmap = serde_json::from_slice(&serialized_product)?;
        trace!("{:#?}", roadmap);
        Ok(Some(roadmap))
    } else {
        Ok(None)
    }
}

