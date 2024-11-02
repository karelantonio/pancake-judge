//! The Judge database
//! Contains a set of services that handle the CRUD of the elements.
//! (ORM is bloat)

pub mod users;

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

/// Errors that may happend querying
#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Unexpected error while executing SQL statement")]
    Sqlx(#[source] sqlx::Error)
}

/// An alias to the backend database used
#[cfg(feature = "use_postgres")]
pub type Pool = sqlx::Pool<sqlx::Postgres>;
#[cfg(not(feature = "use_postgres"))]
pub type Pool = sqlx::Pool<sqlx::Sqlite>;
#[cfg(feature = "use_postgres")]
pub type PoolConnection = sqlx::pool::PoolConnection<Postgres>;
#[cfg(not(feature = "use_postgres"))]
pub type PoolConnection = sqlx::pool::PoolConnection<sqlx::Sqlite>;

/// The database service in general
/// Is the one we ask for the other services
pub struct DatabaseService {
    conn: Pool,
}

impl DatabaseService {
    /// Connect to the given database
    pub async fn new(url: &str) -> Result<Self, ConnectError> {
        Ok(Self {
            conn: Pool::connect(url).await.map_err(ConnectError::Sqlx)?,
        })
    }

    /// Get a new user service
    pub fn users(&self) -> UsersService {
        users::UsersService::new(self.conn.clone())
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
