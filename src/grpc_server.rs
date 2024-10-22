
use crate::services::analyze_specific_repository::analyze_specific_repository;
use crate::models::AppConfig;

use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use tracing::{info, error};
use tracing_futures::Instrument;
use tonic_reflection::server::Builder as ReflectionBuilder;

pub mod analyze {
    tonic::include_proto!("analyze"); // Generated from your proto package
}

#[derive(Debug, Default)]
pub struct AnalyzeService {
    config: Arc<Mutex<AppConfig>>,
}

#[tonic::async_trait]
impl analyze::analyze_server::Analyze for AnalyzeService {
    async fn analyze_repository(
        &self,
        request: Request<analyze::AnalyzeRequest>,
    ) -> Result<Response<analyze::AnalyzeResponse>, Status> {
        let repo_name = request.into_inner().repo_name;

        // Log that we received a request
        info!(repo_name = %repo_name, "Received request to analyze repository");

        let config = self.config.lock().await;
        match analyze_specific_repository(&config, Some(&repo_name)) {
            Ok(_) => {
                info!(repo_name = %repo_name, "Repository analysis successful");
                Ok(Response::new(analyze::AnalyzeResponse {
                    message: format!("Repository {} analyzed successfully", repo_name),
                }))
            }
            Err(e) => {
                error!(repo_name = %repo_name, error = %e, "Repository analysis failed");
                Err(Status::internal(format!("Analysis failed: {}", e)))
            }
        }
    }
}

pub async fn start_grpc_server(config: Arc<Mutex<AppConfig>>) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();

    // Log server startup information
    info!(address = %addr, "Starting gRPC server");

    let analyze_service = AnalyzeService {
        config: Arc::clone(&config),
    };

    let descriptor_set = include_bytes!(concat!(env!("OUT_DIR"), "/analyze_descriptor.bin"));
    // Add reflection service
    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(descriptor_set)
        .build_v1()?;

    Server::builder()
        .add_service(analyze::analyze_server::AnalyzeServer::new(analyze_service))
        .add_service(reflection_service) // Add reflection to the server
        .serve(addr)
        .instrument(tracing::info_span!("grpc_server", addr = %addr)) // Use span to trace the server activity
        .await?;

    // Log server shutdown
    info!("gRPC server has been shut down");

    Ok(())
}


