
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
mod grpc_server;
mod types;

use std::env;
use std::sync::Arc;

use crate::boot::load_config::load_config;
use crate::display_menu::display_menu;
use crate::roadmap::process_yaml_files::process_yaml_files;
use crate::services::analyze_specific_repository::analyze_specific_repository;
use crate::grpc_server::start_grpc_server;
use types::{CustomError, MyError};
use tokio::task::spawn_blocking;
use tracing::{info, error};


#[tokio::main]
async fn main() -> Result<(), MyError> {
    // Load configuration
    let config_file_path = "configuration.yml";
    info!("Loading configuration from {}", config_file_path);

    let config_result = spawn_blocking(move || {
        load_config(config_file_path).unwrap()
    }).await?;

    // Initialize the database
    info!("Initializing the Sled database...");
    let db = match tokio::task::spawn_blocking(|| sled::open("roadmap_db")).await {
        Ok(Ok(db)) => db,
        Ok(Err(e)) => {
            error!("Failed to open the database: {}", e);
            return Err(CustomError::new("Failed to initialize the database"));
        }
        Err(e) => {
            error!("Failed to spawn database initialization task: {}", e);
            return Err(CustomError::new("Failed to initialize the database task"));
        }
    };


    let item_count = db.iter().count();
    info!("Number of items in the database: {}", item_count);

    // Incorporate the DB into the config
    let mut config = config_result;
    config.db = Some(db);
    let shared_config = Arc::new(config);  // Arc-wrapped, no Mutex needed

    // Process YAML files
    info!("Processing YAML files in the roadmap folder...");
    process_yaml_files(&shared_config, &shared_config.roadmap_folder).await?;
    info!("YAML files processed successfully.");

    // Start the gRPC server
    let grpc_config = Arc::clone(&shared_config);
    let grpc_handle = tokio::spawn(async move {
        if let Err(e) = start_grpc_server(grpc_config).await {
            eprintln!("gRPC server failed: {}", e);
        }
    });

    // Handle command-line argument for specific repository analysis
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let repo_name = &args[1];
        analyze_specific_repository(&shared_config, Some(repo_name)).await?;
        return Ok(());
    }

    // Run display menu in a loop
    let menu_handle = tokio::spawn(async move {
        loop {
            let _ = display_menu(&shared_config).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    // Wait for gRPC and menu tasks to complete
    tokio::select! {
        _ = grpc_handle => println!("gRPC server task finished."),
        _ = menu_handle => println!("Menu task finished."),
    };

    Ok(())
}


