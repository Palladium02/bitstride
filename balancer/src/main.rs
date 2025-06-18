mod config;
mod health;

use anyhow::Result;
use config::Config;
use std::env::args;
use tonic::transport::Server;
use crate::health::health::health_server::HealthServer;
use crate::health::HealthService;

#[tokio::main]
async fn main() -> Result<()> {
    match args().skip(1).nth(0) {
        Some(config_path) => {
            let config = Config::from_path(config_path)?;
            let health_service = HealthService::default();

            Server::builder()
                .add_service(HealthServer::new(health_service))
                .serve(config.health_report_address.parse()?).await?;
        }
        None => {
            eprint!(
                "Missing argument for path to config\nUsage: bitstride path/to/stride.toml or cargo run -p balancer -- path/to/stride.toml (when running from source)\n"
            )
        }
    }

    Ok(())
}
