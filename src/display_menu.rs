
use std::error::Error;
use dialoguer::Input;

use crate::services::analyze_all_repositories::analyze_all_repositories;
use crate::services::analyze_project_repositories::analyze_project_repositories;
use crate::services::analyze_specific_repository::analyze_specific_repository;
use crate::services::search_dependency_in_sled::search_dependency_in_sled;
use crate::create_config::AppConfig;

pub fn display_menu(
    config: &AppConfig,
    ) -> Result<(), Box<dyn Error>> {
    let client = &config.client;
    let auth_header = &config.auth_header;
    let url_config = &*config.url_config; // Dereference the Box
    let db = &config.db;

    // Define the menu text and options
    let menu_text = "Select an option:";
    let menu_options = vec![
        "1. Analyser une application (GPECS, XCAD...etc)",
        "2. Analyser un domaine entier (SES, PTEP...etc)",
        "3. Analyser toute les applications",
        "4. Rechercher une exigence dans la roadmap (angular, spring...etc)",
        "5. Exit \n\n",
    ];

    // Combine the text and options dynamically
    let prompt = format!("{}\n{}", menu_text, menu_options.join("\n"));

    // Display the prompt and get the user's input
    let choice:String = Input::new()
        .with_prompt(prompt)
        .interact()?;

    match choice.trim() {
        "1" => {
            analyze_specific_repository(config)?;
        },
        "2" => {
            analyze_project_repositories(config)?;
        },
        "3" => {
            analyze_all_repositories(config)?;
        },
        "4" => {
            search_dependency_in_sled(db)?;
        },
        "5" => {
            tracing::info!("Exiting...");
            std::process::exit(0);
        },
        _ => {
            tracing::warn!("Invalid choice, please try again.");
        }
    }

    Ok(())
}

