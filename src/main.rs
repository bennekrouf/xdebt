
use std::error::Error;
use tracing_subscriber;


mod utils;
mod plugins;
mod roadmap;
mod services;
mod display_menu;
mod url;
mod create_config;
mod kpi;

use roadmap::process_yaml_files::process_yaml_files;
use display_menu::display_menu;
use create_config::create_config;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    let config = create_config()?;
    process_yaml_files(&config.db, "roadmap")?;

    loop {
        // The menu will now handle both input and actions
        if let Err(e) = display_menu(&config) {
            tracing::error!("Error in menu execution: {}", e);
        }
    }
}

