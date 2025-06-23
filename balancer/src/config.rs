use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("No such file")]
    Enoent,
    #[error("Failed to read file")]
    Io,
    #[error("Failed to parse config")]
    Parse,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_address: String,
}

impl Config {
    pub fn from_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut buffer = String::new();
        File::open(path)
            .map_err(|_| ConfigError::Enoent)?
            .read_to_string(&mut buffer)
            .map_err(|_| ConfigError::Io)?;
        Ok(toml::from_str::<Self>(&buffer).map_err(|_| ConfigError::Parse)?)
    }
}
