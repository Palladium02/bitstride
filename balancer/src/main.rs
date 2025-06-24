mod config;
mod health;
mod metric;
mod pool;
mod success;
mod register;
mod priority_queue;

use anyhow::Result;
use config::Config;
use std::env::args;
use std::process::exit;
use std::sync::{Arc, Mutex};
use tonic::transport::Server;
use crate::health::health_rpc::health_server::HealthServer;
use crate::health::HealthService;
use crate::pool::Pool;
use crate::register::register_rpc::register_server::RegisterServer;
use crate::register::RegisterService;

#[tokio::main]
async fn main() -> Result<()> {
    match args().nth(1) {
        Some(config_path) => {
            let config = Config::from_path(config_path)?;
            let pool = Arc::new(Mutex::new(Pool::new()));
            let health_service = HealthService::new(Arc::clone(&pool));
            let register_service = RegisterService::new(Arc::clone(&pool));
            
            Server::builder()
                .add_service(HealthServer::new(health_service))
                .add_service(RegisterServer::new(register_service))
                .serve(config.rpc_address.parse()?).await?;
        }
        None => {
            eprint!(
                "Missing argument for path to config\nUsage: bitstride path/to/stride.toml or cargo run -p balancer -- path/to/stride.toml (when running from source)\n"
            );
            exit(1);
        }
    }

    Ok(())
}
