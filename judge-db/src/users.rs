//! The users service, handles all the operations related to user, see [`UsersService`]

use sqlx::pool::PoolConnection;
use sqlx::{query, Any};
use sqlx::{AnyConnection, Connection};

/// A structure that maps a user from a database row
/// A user must contain at least it's ID, Username, Name, Email, PasswordHash, PasswordSalt
/// Also, there must be another table that contains at which groups this user belongs.
///
/// When deleting a user, we MUST also delete its relations: Insitutions
#[derive(Clone, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub name: String,
    pub email: String,
    pub pass_hash: String,
    pub pass_salt: String,
}

/// A users service
/// This service should be responsible for making the requests to the database
/// and return mapped structs (See [`User`])
/// By wrapping this logic in a service instead of using ORMs we have more flexibility
/// in the queries and less dependencies.
pub struct UsersService {
    conn: PoolConnection<Any>,
}

impl UsersService {
    /// Default constructor, connects to the given address
    pub async fn new(conn: PoolConnection<Any>) -> Self {
        Self { conn }
    }

    /// Query all users
    pub async fn query_all_users(&self) {}
}
