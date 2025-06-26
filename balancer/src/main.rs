mod config;
mod health;
mod metric;
mod pool;
mod success;
mod register;
mod priority_queue;
mod proxy;
mod balancer;
mod grpc;

use anyhow::Result;
use config::Config;
use std::env::args;
use std::process::exit;
use crate::balancer::Balancer;

#[tokio::main]
async fn main() -> Result<()> {
    match args().nth(1) {
        Some(config_path) => {
            let config = Config::from_path(config_path)?;
            let _ = Balancer::new(config).run().await;
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
