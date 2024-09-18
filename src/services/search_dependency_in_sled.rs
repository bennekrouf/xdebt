
use dialoguer::Input;
use std::error::Error;

use crate::services::get_product_version::get_product_version;

// Function to search for a dependency in Sled DB
pub fn search_dependency_in_sled(db: &sled::Db) -> Result<(), Box<dyn Error>> {
    // Prompt user for dependency name
    let dependency: String = Input::new()
        .with_prompt("Enter the dependency name (e.g., angular)")
        .interact()?;

    // Get the dependency from the Sled database
    match get_product_version(db, &dependency)? {
        Some(product_version) => {
            // Display the found product version information
            println!("Dependency found: {:#?}", product_version);
        },
        None => {
            println!("No entry found for dependency: {}", dependency);
        }
    }

    Ok(())
}
