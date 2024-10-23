use dialoguer::Input;
use std::error::Error;

use crate::models::AppConfig;
use crate::services::analyze_all_repositories::analyze_all_repositories;
use crate::services::analyze_specific_project::analyze_specific_project;
use crate::services::analyze_specific_repository::analyze_specific_repository;
use crate::services::search_dependency_in_sled::search_dependency_in_sled;

pub async fn display_menu(config: &AppConfig) -> Result<(), Box<dyn Error + Send + Sync>> {
    let db = &config.db.as_ref().expect("Db should be initialized");

    // Define the menu text and options
    let menu_text = "Selectionne une     let cloned_config = config1.clone();option:";
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
    let choice: String = Input::new().with_prompt(prompt).interact()?;

    match choice.trim() {
        "1" => {
            let _ = analyze_specific_repository(config, None).await;
        }
        "2" => {
            let _ = analyze_specific_project(config).await;
        }
        "3" => {
            let _ = analyze_all_repositories(config).await;
        }
        "4" => {
            let _ = search_dependency_in_sled(db).await;
        }
        "5" => {
            tracing::info!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            tracing::warn!("Invalid choice, please try again.");
        }
    }

    Ok(())
}
