
use serde_json;
use sled;
use std::error::Error;

use crate::roadmap::models::ProductVersion;
// Retrieve a product from sled DB
pub fn retrieve_from_sled(db: &sled::Db, product: &str) -> Result<Option<ProductVersion>, Box<dyn Error>> {
    if let Some(serialized_product) = db.get(product)? {
        let product_version: ProductVersion = serde_json::from_slice(&serialized_product)?;
        println!("{:#?}", product_version);
        Ok(Some(product_version))
    } else {
        Ok(None)
    }
}

