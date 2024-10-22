
use dialoguer::Input;
use std::error::Error;
use tracing::info;
use crate::services::get_roadmap::get_roadmap;

// Function to search for a product in Sled DB
pub fn search_dependency_in_sled(db: &sled::Db) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Prompt user for product name
    let product: String = Input::new()
        .with_prompt("Enter the product name (e.g., angular)")
        .interact()?;

    // Get the product from the Sled database
    match get_roadmap(db, &product)? {
        Some(roadmap) => {
            // Display the found product version information
            info!("Dependency found: {:#?}", roadmap);
        },
        None => {
            info!("No entry found for product: {}", product);
        }
    }

    Ok(())
}
