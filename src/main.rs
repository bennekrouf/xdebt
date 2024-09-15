
use std::error::Error;
use std::env;
use tracing_subscriber;

mod utils;
mod plugins;
mod roadmap;
mod services;
mod display_menu;

use utils::create_client_with_auth::create_client_with_auth;
use crate::roadmap::process_yaml_files::process_yaml_files;
use crate::display_menu::display_menu;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();

    // Open the Sled database - Process all YAML files in the 'roadmap' folder
    let db = sled::open("roadmap_db")?;
    process_yaml_files(&db, "roadmap")?;

    let (client, auth_header) = create_client_with_auth()?;
    let repos_url_template = env::var("REPOS_URL")
        .map_err(|e| format!("Missing REPOS_URL environment variable: {}", e))?;

    loop {
        // The menu will now handle both input and actions
        if let Err(e) = display_menu(&client, &auth_header, &repos_url_template) {
            tracing::error!("Error in menu execution: {}", e);
        }
    }
}

