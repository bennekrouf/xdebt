
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
use tokio::sync::Mutex;

use crate::boot::load_config::load_config;
use crate::display_menu::display_menu;
use crate::roadmap::process_yaml_files::process_yaml_files;
use crate::services::analyze_specific_repository::analyze_specific_repository;
use crate::grpc_server::start_grpc_server;
use types::{CustomError, MyError};
// use models::AppConfig;
use tokio::task::spawn_blocking;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), MyError> {
    // Load configuration
    let config_file_path = "configuration.yml";
    info!("Loading configuration from {}", config_file_path);

    let config_result = spawn_blocking(move || {
        load_config(config_file_path).unwrap()
    }).await;


    // Initialize the database
    info!("Initializing the Sled database...");
    let db_result = tokio::task::spawn_blocking(|| sled::open("roadmap_db")).await;

     // Handle database initialization failure
    let db = match db_result {
        Ok(Ok(db)) => db, // Database opened successfully
        Ok(Err(e)) => {
            error!("Failed to open the database: {}", e);
            return Err(CustomError::new("Failed to initialize the database"));  // Early exit if database initialization fails
        }
        Err(e) => {
            error!("Failed to spawn database initialization task: {}", e);
            return Err(CustomError::new("Failed to initialize the database task"));  // Early exit if the task itself fails
        }
    };

    let item_count = db.iter().count();
    info!("Number of items in the database: {}", item_count);

    let shared_config = Arc::new(Mutex::new(config_result?));
    {
        let mut config = shared_config.lock().await; // Lock the Mutex to get a mutable reference
        config.db = Some(db); // Set the database
    }
    info!("Database initialized successfully.");

    info!("Configuration loaded successfully.");

    {
        let config = shared_config.lock().await;
        info!("Processing YAML files in the roadmap folder...");
        let _ = process_yaml_files(&config, &config.roadmap_folder).await;  // Synchronous call
        info!("YAML files processed successfully.");
    }

    // Start watching the config file for changes
    // watch_config_for_reload(Arc::clone(&shared_config))?;

    // Start the gRPC server in a non-blocking async task
    let grpc_config = Arc::clone(&shared_config);
    // let grpc_handle = tokio::spawn(async move {
    //     let config = grpc_config.lock().await;
    //     let config_clone = Arc::new(Mutex::new(config.clone()));
    //     if let Err(e) = start_grpc_server(config_clone).await {
    //         eprintln!("gRPC server failed: {}", e);
    //     }
    // });

    // Check command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If an argument is passed, trigger specific analysis asynchronously
        let repo_name = &args[1];
        let config = shared_config.lock().await;
        analyze_specific_repository(&config, Some(repo_name)).await?;
        return Ok(());
    }

    // If no argument is passed, proceed with showing the menu
    // let mut yaml_processed = false;

    let menu_handle = tokio::spawn(async move {
        loop {
            // let config_clone = Arc::clone(&shared_config);  // Clone the Arc for process_yaml_files task

            // if !yaml_processed {
            //     // Wrap blocking code in `spawn_blocking` and return a `Result`
            //     let _ = tokio::spawn(async move {
            //         let config = config_clone.lock().await;
            //         process_yaml_files(&config, &config.roadmap_folder);
            //     }).await; // Properly unwrap Result
            //     yaml_processed = true;
            // }

            let config_clone = Arc::clone(&shared_config);  // Clone the Arc for display_menu task

            let _ = tokio::spawn(async move {
                let config1 = config_clone.lock().await;
                // let cloned_config = config1.clone();
                display_menu(&config1).await
            }).await;

            // Yield control back to the async runtime to avoid blocking
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    // Wait for all tasks to complete to prevent premature shutdown
    // let _ = tokio::select! {
        // _ = grpc_handle => {
        //     println!("gRPC server task finished.");
        // }
        // _ = menu_handle => {
        //     println!("Menu task finished.");
        // }
    // };

    Ok(())
}

