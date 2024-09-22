use std::error::Error;
use tracing_subscriber;

mod boot;
mod display_menu;
mod kpi;
mod models;
mod plugins;
mod roadmap;
mod services;
mod url;
mod utils;

use boot::load_config::load_config;
use boot::watch_config_for_reload::watch_config_for_reload;
use display_menu::display_menu;
use roadmap::process_yaml_files::process_yaml_files;
use std::sync::{Arc, Mutex};
use tracing_subscriber::filter::EnvFilter;

fn main() -> Result<(), Box<dyn Error>> {
    // Initial configuration load
    let config_file_path = "configuration.yml";
    let mut config = load_config(config_file_path)?;

    // Ensure the tracing subscriber is initialized only once
    static INIT: std::sync::Once = std::sync::Once::new();

    let env_filter = EnvFilter::new(
        "trace".to_owned()                            // Enable trace-level logs globally
        + ",sled::pagecache=info"
        + ",sled::tree=info"
        + ",reqwest::blocking::wait=info"
        + ",sled::pagecache::iobuf=info", // Set sled::pagecache::iobuf to info (hides trace)
    );

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(config.trace_level)
            .with_env_filter(env_filter)
            .init();
    });

    let db = sled::open("roadmap_db")?;
    config.db = Some(db);

    // Shared configuration wrapped in Arc and Mutex for thread-safe updates
    let shared_config = Arc::new(Mutex::new(config));

    // Start watching the config file for changes
    watch_config_for_reload(Arc::clone(&shared_config))?;

    // Track if YAML files have been processed
    let mut yaml_processed = false;

    loop {
        // Use the shared configuration for your logic
        let config = shared_config.lock().unwrap();
        let db = config.db.as_ref().ok_or("Database is not initialized")?;

        // Only process YAML files once
        if !yaml_processed {
            process_yaml_files(db, "roadmap")?;
            yaml_processed = true; // Set to true after processing
        }

        // Handle menu display and actions
        if let Err(e) = display_menu(&config) {
            tracing::error!("Error in menu execution: {}", e);
        }

        // Optional: Check if configuration has changed and reset yaml_processed if necessary
        // You can implement a method to check for changes in the config
        // if config_changed {
        //     yaml_processed = false; // Reset flag to allow reprocessing
        // }
    }
}
