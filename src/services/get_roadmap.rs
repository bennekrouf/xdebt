
use serde_json;
use sled;
use std::error::Error;

use crate::models::Roadmap;

// Get a dependency version from sled DB (case-insensitive)
pub fn get_roadmap(db: &sled::Db, dependency: &str) -> Result<Option<Roadmap>, Box<dyn Error>> {
    // Convert the dependency key to lowercase for case-insensitive search
    let dependency_lower = dependency.to_lowercase();

    if let Some(serialized_product) = db.get(dependency_lower.as_bytes())? {
        let roadmap: Roadmap = serde_json::from_slice(&serialized_product)?;
        println!("{:#?}", roadmap);
        Ok(Some(roadmap))
    } else {
        Ok(None)
    }
}

