mod boot;
mod consume_messages;
mod display_menu;
mod kpi;
mod models;
mod plugins;
mod roadmap;
mod services;
mod url;
mod utils;

use anyhow::Result;
use consume_messages::consume_messages;
use iggy::client::{Client, UserClient};
use iggy::clients::builder::IggyClientBuilder;
use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};
// use tracing_subscriber::filter::EnvFilter;
// use tracing_subscriber;
use crate::boot::load_config::load_config;
use crate::boot::watch_config_for_reload::watch_config_for_reload;
use crate::display_menu::display_menu;
use crate::roadmap::process_yaml_files::process_yaml_files;
use crate::services::analyze_specific_repository::analyze_specific_repository;
//use tokio::task;

use crate::models::AppConfig;
use iggy::clients::client::IggyClient;

//#[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = "configuration.yml";
    let mut config = load_config(config_file_path)?;

    // Initialize tracing subscriber
    // static INIT: std::sync::Once = std::sync::Once::new();
    // let env_filter = EnvFilter::new(format!(
    //     "{},hyper=trace,hyper_util::client::legacy::pool=trace,sled::pagecache=info,sled::tree=info,reqwest::blocking::wait=info,sled::pagecache::iobuf=info",
    //     config.trace_level
    // ));
    //
    // INIT.call_once(|| {
    //     tracing_subscriber::fmt()
    //         .with_max_level(config.trace_level)
    //         .with_env_filter(env_filter)
    //         .init();
    // });

    let db = sled::open("roadmap_db")?;
    config.db = Some(db);

    let shared_config = Arc::new(Mutex::new(&config));

    // Start watching the config file for changes
    //watch_config_for_reload(Arc::clone(&shared_config))?;

    // Check command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If an argument is passed (like "gpecs"), trigger specific analysis
        let repo_name = &args[1];
        let config = shared_config.lock().unwrap();
        analyze_specific_repository(&config, Some(repo_name))?;
        return Ok(());
    }

    // Create single Iggy handler with one runtime
    let iggy_handler = IggyHandler::new()?;

    // Connect to Iggy
    let client = iggy_handler.connect_iggy()?;

    // Setup shutdown channel
    let (_shutdown_tx, shutdown_rx) = oneshot::channel();

    println!("Starting consumer...");

    // Run the consumer using the same runtime
    iggy_handler.run_consumer(&config, &client, shutdown_rx)?;
    let mut yaml_processed = false;

    loop {
        let config = shared_config.lock().unwrap();
        let db = config.db.as_ref().ok_or("Database is not initialized")?;

        if !yaml_processed {
            process_yaml_files(db, "roadmap")?;
            yaml_processed = true;
        }

        if let Err(e) = display_menu(&config) {
            tracing::error!("Error in menu execution: {}", e);
        }
    }
}

use tokio::runtime::Runtime;
use tokio::sync::oneshot;
struct IggyHandler {
    runtime: Runtime,
}

impl IggyHandler {
    fn new() -> Result<Self> {
        Ok(Self {
            runtime: Runtime::new()?,
        })
    }

    fn connect_iggy(&self) -> Result<IggyClient> {
        self.runtime.block_on(async {
            let client = IggyClientBuilder::new()
                .with_tcp()
                .with_server_address("abjad.mayorana.ch:8090".to_string())
                .build()?;

            client.connect().await?;
            client.login_user("iggy", "iggy").await?;

            Ok(client)
        })
    }

    fn run_consumer(
        &self,
        config: &AppConfig,
        client: &IggyClient,
        shutdown: oneshot::Receiver<()>,
    ) -> Result<()> {
        self.runtime.block_on(async {
            let tenant = "gibro";
            let topic = "notification";

            tokio::select! {
                _ = shutdown => {
                    println!("Shutting down consumer...");
                    Ok(())
                }
                result = consume_messages(&config, client, tenant, topic) => {
                    Ok(result?)
                }
            }
        })
    }
}
