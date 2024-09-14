
use serde_json;
use sled;
use std::error::Error;

use crate::roadmap::models::Versions;
// Persist each product version to sled DB
pub fn persist_to_sled(db: &sled::Db, versions: &Versions) -> Result<(), Box<dyn Error>> {
    for product_version in &versions.versions {
        let serialized_product = serde_json::to_vec(&product_version)?;
        db.insert(product_version.product.as_bytes(), serialized_product)?;
    }
    db.flush()?;
    Ok(())
}

