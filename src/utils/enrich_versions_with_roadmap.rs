
use sled::Db;
use tracing::{info, trace};
use crate::services::get_roadmap::get_roadmap;
use crate::models::{Roadmap, Analysis};
use crate::types::MyError;

pub async fn enrich_versions_with_roadmap(
     db: &Db,
     analyses: Vec<Analysis>,
 ) -> Result<Vec<Analysis>, MyError> {
    let mut enriched_analyses = analyses;
    // Iterate over the mutable reference of analyses to enrich them
    for analysis in enriched_analyses.iter_mut() {
        // Extract the product name from the `dependency_version`
        let product = &analysis.dependency_version.product;
        trace!("Looking for roadmap for product : {}", product);

        // Get the roadmap for the product from the sled DB
        let roadmap = match get_roadmap(&db, product)? {
            Some(roadmap) => roadmap,  // Return the Roadmap struct
            None => {
                info!("No roadmap found for product : {}", product);
                // Handle missing roadmaps by creating a default or empty roadmap
                Roadmap {
                    product: product.to_string(),
                    domain: None,
                    chapter: None,
                    entries: vec![],
                }
            }
        };

        trace!("Enriching roadmap with : {} {:?}", product, roadmap);

        // Enrich the existing `analysis` with the fetched roadmap
        analysis.roadmap = Some(roadmap);
    }

    Ok(enriched_analyses)
}

