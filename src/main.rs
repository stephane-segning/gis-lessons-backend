use crate::modules::api::handler::ApiService;
use crate::modules::db::connection::get_connection;
use crate::modules::env::env::EnvConfig;
use crate::modules::router::router::router;
use crate::modules::tracer::init::init_tracing;
use envconfig::Envconfig;
use opentelemetry::global;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::{debug, info};

mod domain;
mod modules;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EnvConfig::init_from_env()?;

    // Initialize tracing and OpenTelemetry
    let metrics = init_tracing(config.clone()).await?;
    let db_pool = get_connection(config.clone()).await?;
    let db_pool = Arc::new(RwLock::new(db_pool));

    // Get address to listen on
    let addr = format!("{}:{:?}", config.http_host, config.http_port).parse::<SocketAddr>()?;
    let listener = TcpListener::bind(addr).await?;
    debug!(config.http_port, config.http_host, "Will start");

    let api_service = Arc::new(ApiService::create(db_pool)?);

    let app = router(metrics, api_service).await?;

    // Start the server
    info!("Server running on http://{:?}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    // Shutdown the tracer provider
    global::shutdown_tracer_provider();
    Ok(())
}
