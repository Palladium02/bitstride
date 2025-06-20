mod config;
mod health;
mod metric;
mod pool;
mod success;
mod register;

use anyhow::Result;
use config::Config;
use std::env::args;
use tonic::transport::Server;
use crate::health::health_rpc::health_server::HealthServer;
use crate::health::HealthService;
use crate::register::register_rpc::register_server::RegisterServer;
use crate::register::RegisterService;

#[tokio::main]
async fn main() -> Result<()> {
    match args().nth(1) {
        Some(config_path) => {
            let config = Config::from_path(config_path)?;
            let health_service = HealthService;
            let register_service = RegisterService;
            
            Server::builder()
                .add_service(HealthServer::new(health_service))
                .add_service(RegisterServer::new(register_service))
                .serve(config.rpc_address.parse()?).await?;
        }
        None => {
            eprint!(
                "Missing argument for path to config\nUsage: bitstride path/to/stride.toml or cargo run -p balancer -- path/to/stride.toml (when running from source)\n"
            )
        }
    }

    Ok(())
}
