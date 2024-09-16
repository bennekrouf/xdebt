
use dialoguer::Input;
use crate::roadmap::retrieve_from_sled::retrieve_from_sled;
use std::error::Error;

// Function to search for a dependency in Sled DB
pub fn search_dependency_in_sled(db: &sled::Db) -> Result<(), Box<dyn Error>> {
    // Prompt user for dependency name
    let dependency: String = Input::new()
        .with_prompt("Enter the dependency name (e.g., angular)")
        .interact()?;

    // Retrieve the dependency from the Sled database
    match retrieve_from_sled(db, &dependency)? {
        Some(product_version) => {
            // Display the retrieved product version information
            println!("Dependency found: {:#?}", product_version);
        },
        None => {
            println!("No entry found for dependency: {}", dependency);
        }
    }

    Ok(())
}
