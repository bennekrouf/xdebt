
use serde_json;
use sled;
use std::error::Error;
use std::collections::HashSet;

use crate::models::Roadmap;

// Get distinct list of dependencies from sled DB
pub fn get_distinct_dependencies(db: &sled::Db) -> Result<Vec<String>, Box<dyn Error>> {
    let mut dependency_set = HashSet::new();

    for item in db.iter() {
        let (_, serialized_product) = item?;
        let roadmap: Roadmap = serde_json::from_slice(&serialized_product)?;

        // Insert product into the set (to ensure distinct values)
        dependency_set.insert(roadmap.product);
    }

    // Convert HashSet to a Vec and return
    Ok(dependency_set.into_iter().collect())
}
