//! The Judge database
//! Contains a set of services that handle the CRUD of the elements.
//! (ORM is bloat)

pub mod users;

use sqlx::AnyPool;
use thiserror::Error;
use users::UsersService;

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

/// Errors that may happend while getting a new service
#[derive(Error, Debug)]
pub enum NewServiceError {
    #[error("The driven returned an error while getting a connection from the pool")]
    Sqlx(#[source] sqlx::Error),
}

/// The database service in general
/// Is the one we ask for the other services
pub struct DatabaseService {
    conn: AnyPool,
}

impl DatabaseService {
    /// Connect to the given database
    pub async fn new(url: &str) -> Result<Self, ConnectError> {
        Ok(Self {
            conn: AnyPool::connect(url).await.map_err(ConnectError::Sqlx)?,
        })
    }

    /// Get a new user service
    pub async fn users(&self) -> Result<UsersService, NewServiceError> {
        Ok(
            users::UsersService::new(self.conn.acquire().await.map_err(NewServiceError::Sqlx)?)
                .await,
        )
    }

    /// Runs the migrations
    pub async fn run_migrations(&self) -> Result<(), MigrateError> {
        sqlx::migrate!("./migrations")
            .run(&self.conn)
            .await
            .map_err(MigrateError::Sqlx)?;
        Ok(())
    }

    /// Sets up the driver
    pub fn setup_drivers() {
        sqlx::any::install_default_drivers();
    }
}
