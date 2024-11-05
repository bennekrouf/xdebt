// use anyhow::{Context, Result};
use futures::TryStreamExt;
use iggy::clients::client::IggyClient;
use iggy::clients::consumer::ReceivedMessage;  // Add this import
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};
use crate::models::AppConfig;
use crate::services::analyze_specific_repository::analyze_specific_repository;
// use crate::types::CustomError;
use anyhow::{anyhow, Context, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePayload {
    timestamp: String,
    action: String,
    parameters: Vec<String>,
}

pub async fn consume_messages(
    config: &AppConfig,
    client: &IggyClient,
    tenant: &str,
    topic: &str,
) -> Result<()> {
    let mut consumer = client
        .consumer_group("display_group", tenant, topic)
        .context("Failed to create consumer group")?
        .create_consumer_group_if_not_exists()
        .auto_join_consumer_group()
        .build();

    consumer.init().await.context("Failed to initialize consumer")?;

    info!(
        "Started consuming messages from tenant: {}, topic: {}",
        tenant, topic
    );

    while let Ok(Some(message)) = consumer.try_next().await {
        if let Err(e) = process_message(&message, config, tenant, topic).await {
            error!("Error processing message: {:#}", e);
            continue;
        }
    }

    Ok(())
}

async fn process_message(
    message: &ReceivedMessage,
    config: &AppConfig,
    tenant: &str,
    topic: &str,
) -> Result<()> {
    let payload = String::from_utf8(message.message.payload.to_vec())
        .context("Failed to decode message payload")?;

    let message_payload: MessagePayload = serde_json::from_str(&payload)
        .context("Failed to parse message JSON")?;

    println!("\n=== Message Received ===");
    println!("Tenant (Stream): {}", tenant);
    println!("Topic: {}", topic);
    println!("Action: {}", message_payload.action);
    println!("Parameters:");
    for (index, param) in message_payload.parameters.iter().enumerate() {
        println!("  {}: {}", index + 1, param);
    }
    println!("Timestamp: {}", message_payload.timestamp);
    println!("=====================\n");

    match message_payload.action.as_str() {
        "analyze_specific_repository" => {
            process_analyze_repository(&message_payload, config).await?;
        }
        unknown_action => {
            warn!("Unknown action received: {}", unknown_action);
            println!("Unknown action: {}", unknown_action);
        }
    }

    Ok(())
}

async fn process_analyze_repository(
    message_payload: &MessagePayload,
    config: &AppConfig,
) -> Result<()> {
    let repo_name = if let Some(name) = message_payload.parameters.first() {
        name
    } else {
        return Err(anyhow!("No repository name provided"));
    };

    match analyze_specific_repository(config, Some(repo_name)).await {
        Ok(_) => {
            info!("Successfully analyzed repository: {}", repo_name);
            println!("Successfully analyzed repository: {}", repo_name);
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to analyze repository {}: {}", repo_name, e);
            error!("{}", error_msg);
            println!("{}", error_msg);
            Err(anyhow::anyhow!(error_msg))
        }
    }
}
