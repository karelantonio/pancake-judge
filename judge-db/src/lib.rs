//! The Judge database
//! Contains a set of services that handle the CRUD of the elements.
//! (ORM is bloat)

pub mod users;

use sqlx::AnyPool;
use thiserror::Error;

/// Errors that may happen while connecting to the database
#[derive(Error, Debug)]
pub enum ConnectError {
    #[error("The driver returned an error while connecting to the database")]
    Sqlx(#[source] sqlx::Error),
}

/// Errors that may happen while migrating the database
#[derive(Error, Debug)]
pub enum MigrateError {
    #[error("Driver returned an error while running migrations")]
    Sqlx(#[source] sqlx::migrate::MigrateError),
}

/// Runs the migrations
pub async fn run_migrations(db: &AnyPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations")
        .run(db)
        .await
        .map_err(MigrateError::Sqlx)?;
    Ok(())
}
