mod config;

use anyhow::Result;
use config::Config;
use std::env::args;

fn main() -> Result<()> {
    match args().skip(1).nth(0) {
        Some(config_path) => {
            let _ = Config::from_path(config_path)?;
        }
        None => {
            eprint!(
                "Missing argument for path to config\nUsage: bitstride path/to/stride.toml or cargo run -p balancer -- path/to/stride.toml (when running from source)\n"
            )
        }
    }

    Ok(())
}
