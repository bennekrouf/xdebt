
use serde_json;
use sled;
use std::error::Error;

use crate::roadmap::models::ProductVersion;

// Get a product version from sled DB (case-insensitive)
pub fn get_product_version(db: &sled::Db, product: &str) -> Result<Option<ProductVersion>, Box<dyn Error>> {
    // Convert the product key to lowercase for case-insensitive search
    let product_lower = product.to_lowercase();

    if let Some(serialized_product) = db.get(product_lower.as_bytes())? {
        let product_version: ProductVersion = serde_json::from_slice(&serialized_product)?;
        println!("{:#?}", product_version);
        Ok(Some(product_version))
    } else {
        Ok(None)
    }
}

