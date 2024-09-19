
use dialoguer::Input;
use std::error::Error;

use crate::services::get_roadmap::get_roadmap;

// Function to search for a dependency in Sled DB
pub fn search_dependency_in_sled(db: &sled::Db) -> Result<(), Box<dyn Error>> {
    // Prompt user for dependency name
    let dependency: String = Input::new()
        .with_prompt("Enter the dependency name (e.g., angular)")
        .interact()?;

    // Get the dependency from the Sled database
    match get_roadmap(db, &dependency)? {
        Some(roadmap) => {
            // Display the found product version information
            println!("Dependency found: {:#?}", roadmap);
        },
        None => {
            println!("No entry found for dependency: {}", dependency);
        }
    }

    Ok(())
}
