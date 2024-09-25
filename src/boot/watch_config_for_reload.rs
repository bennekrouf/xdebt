
use notify::{Watcher, EventKind, RecommendedWatcher, RecursiveMode, Config};
use std::sync::mpsc::channel;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::path::Path;
use std::thread;
use std::env;

use crate::models::AppConfig;
use crate::boot::load_config::load_config;

// Set up hot-reload using a file watcher
pub fn watch_config_for_reload(shared_config: Arc<Mutex<AppConfig>>) -> Result<(), Box<dyn Error>> {
    let config_file_path = "configuration.yml";
    let (tx, rx) = channel();

    // Move the watcher into a new thread to keep it alive
    thread::spawn(move || {
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).expect("Failed to create watcher");
        watcher.watch(Path::new(config_file_path), RecursiveMode::NonRecursive).expect("Failed to watch file");

        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    match event.kind {
                        EventKind::Modify(_) | EventKind::Create(_) => {
                            if let Err(e) = load_and_update_config(config_file_path, &shared_config) {
                                tracing::error!("Failed to reload config: {}", e);
                            }
                        }
                        _ => {} // Ignore other events
                    }
                }
                Ok(Err(e)) => {
                    tracing::error!("File event error: {:?}", e);
                }
                Err(e) => {
                    tracing::error!("Channel receive error: {:?}", e);
                    break; // Exit the loop if there's an error
                }
            }
        }
    });

    Ok(())
}

fn load_and_update_config(config_file_path: &'static str, shared_config: &Arc<Mutex<AppConfig>>) -> Result<(), Box<dyn Error>> {
    log_working_directory(); 
    // Check if the file still exists
    if !Path::new(config_file_path).exists() {
        return Err(format!("Configuration file '{}' not found.", config_file_path).into());
    }

    let new_config = load_config(config_file_path)?;
    let mut config = shared_config.lock().unwrap();
    *config = new_config;
    tracing::info!("Configuration reloaded.");
    Ok(())
}


fn log_working_directory() {
    match env::current_dir() {
        Ok(path) => tracing::info!("Current working directory: {}", path.display()),
        Err(e) => tracing::error!("Failed to get current working directory: {}", e),
    }
}


