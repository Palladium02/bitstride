use sqlx::SqlitePool;
use sqlx::migrate::Migrator;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("Failed to create database file")]
    CreationError,
    #[error("Failed to establish connection")]
    ConnectionError,
    #[error("Failed to run migrations")]
    MigrationError,
}

pub(crate) struct PersistenceService {
    pool: SqlitePool,
}

impl PersistenceService {
    pub async fn new(url: &str) -> Result<Self, PersistenceError> {
        if !Path::new(url).exists() {
            File::create(url).map_err(|_| PersistenceError::CreationError)?;
        }

        let pool = SqlitePool::connect(url)
            .await
            .map_err(|_| PersistenceError::ConnectionError)?;
        let migrator: Migrator = sqlx::migrate!("./migrations");

        migrator
            .run(&pool)
            .await
            .map_err(|_| PersistenceError::MigrationError)?;

        Ok(Self { pool })
    }
}
