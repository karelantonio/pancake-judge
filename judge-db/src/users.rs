//! The users service, handles all the operations related to user, see [`UsersService`]

use crate::Pool;
use sqlx::query_as;

/// A structure that maps a user from a database row
/// A user must contain at least it's ID, Username, Name, Email, PasswordHash, PasswordSalt
/// Also, there must be another table that contains at which groups this user belongs.
///
/// When deleting a user, we MUST also delete its relations: Insitutions
#[derive(Clone, Debug)]
pub struct User {
    pub id: i64, // See the SQLite docs for ROWID
    pub username: String,
    pub name: String,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
    pub join_date: Option<chrono::NaiveDateTime>,
    pub problems: i64,
}

/// An alias to the database type used

/// A users service
/// This service should be responsible for making the requests to the database
/// and return mapped structs (See [`User`])
/// By wrapping this logic in a service instead of using ORMs we have more flexibility
/// in the queries and less dependencies.
pub struct UsersService {
    conn: Pool,
}

impl UsersService {
    /// Default constructor, connects to the given address
    pub fn new(conn: Pool) -> Self {
        Self { conn }
    }

    /// Query all users
    pub async fn query_all_users(&self) -> Result<Vec<User>, crate::QueryError> {
        let q = query_as!(User, "SELECT * FROM users");
        let re = q.fetch_all(&self.conn).await;
        re.map_err(crate::QueryError::Sqlx)
    }
}
