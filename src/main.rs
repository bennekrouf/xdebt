
use std::error::Error;
use tracing_subscriber;

mod boot;
mod utils;
mod plugins;
mod roadmap;
mod services;
mod display_menu;
mod url;
mod kpi;
mod models;

use roadmap::process_yaml_files::process_yaml_files;
use display_menu::display_menu;
use boot::load_config::load_config;
use boot::watch_config_for_reload::watch_config_for_reload;

use std::sync::{Arc, Mutex};
// use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Ensure the tracing subscriber is initialized only once
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
    });

    // Initial configuration load
    let config_file_path = "configuration.yml";
    let mut config = load_config(config_file_path)?;

    let db = sled::open("roadmap_db")?;
    config.db = Some(db);

    // Shared configuration wrapped in Arc and Mutex for thread-safe updates
    let shared_config = Arc::new(Mutex::new(config));

    // Start watching the config file for changes
    watch_config_for_reload(Arc::clone(&shared_config))?;

    loop {
        // Use the shared configuration for your logic
        let config = shared_config.lock().unwrap();

        let db = config.db.as_ref().ok_or("Database is not initialized")?;
        process_yaml_files(db, "roadmap")?;

        // Handle menu display and actions
        if let Err(e) = display_menu(&config) {
            tracing::error!("Error in menu execution: {}", e);
        }
    }
}

