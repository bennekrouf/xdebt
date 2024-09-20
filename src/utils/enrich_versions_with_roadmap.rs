
use sled::Db;
use std::error::Error;
use crate::services::get_roadmap::get_roadmap;
use crate::models::{Roadmap, Analysis};

pub fn enrich_versions_with_roadmap(
     db: &Db,
     analyses: Vec<Analysis>,
 ) -> Result<Vec<Analysis>, Box<dyn Error>> {
    let mut enriched_analyses = analyses;
    // Iterate over the mutable reference of analyses to enrich them
    for analysis in enriched_analyses.iter_mut() {
        // Extract the dependency name from the `dependency_version`
        let dependency_name = &analysis.dependency_version.dependency_name;

        // Get the roadmap for the dependency from the sled DB
        let roadmap = match get_roadmap(&db, dependency_name)? {
            Some(roadmap) => roadmap,  // Return the Roadmap struct
            None => {
                // Handle missing roadmaps by creating a default or empty roadmap
                Roadmap {
                    dependency: dependency_name.to_string(),
                    domain: None,
                    chapter: None,
                    entries: vec![],
                }
            }
        };

        // Enrich the existing `analysis` with the fetched roadmap
        analysis.roadmap = Some(roadmap);
    }

    Ok(enriched_analyses)
}

