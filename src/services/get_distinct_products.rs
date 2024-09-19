
use serde_json;
use sled;
use std::error::Error;
use std::collections::HashSet;

use crate::models::Roadmap;

// Get distinct list of products from sled DB
pub fn get_distinct_products(db: &sled::Db) -> Result<Vec<String>, Box<dyn Error>> {
    let mut product_set = HashSet::new();

    for item in db.iter() {
        let (_, serialized_product) = item?;
        let product_version: Roadmap = serde_json::from_slice(&serialized_product)?;

        // Insert product into the set (to ensure distinct values)
        product_set.insert(product_version.product);
    }

    // Convert HashSet to a Vec and return
    Ok(product_set.into_iter().collect())
}
