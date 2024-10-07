
mod boot;
mod display_menu;
mod kpi;
mod models;
mod plugins;
mod roadmap;
mod services;
mod url;
mod utils;
mod fetch_repositories;

use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};

use crate::boot::load_config::load_config;
use crate::boot::watch_config_for_reload::watch_config_for_reload;
use crate::display_menu::display_menu;
use crate::roadmap::process_yaml_files::process_yaml_files;
use crate::services::analyze_specific_repository::analyze_specific_repository;

fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = "configuration.yml";
    let mut config = load_config(config_file_path)?;

    let db = sled::open("roadmap_db")?;
    config.db = Some(db);

    let shared_config = Arc::new(Mutex::new(config));

    // Start watching the config file for changes
    watch_config_for_reload(Arc::clone(&shared_config))?;

    // Check command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If an argument is passed (like "gpecs"), trigger specific analysis
        let repo_name = &args[1];
        let config = shared_config.lock().unwrap();
        analyze_specific_repository(&config, Some(repo_name))?;
        return Ok(());
    }

    // If no argument is passed, proceed with showing the menu
    let mut yaml_processed = false;

    loop {
        let config = shared_config.lock().unwrap();

        if !yaml_processed {
            process_yaml_files(&config, &config.roadmap_folder)?;
            yaml_processed = true;
        }

        if let Err(e) = display_menu(&config) {
            tracing::error!("Error in menu execution: {}", e);
        }
    }
}

