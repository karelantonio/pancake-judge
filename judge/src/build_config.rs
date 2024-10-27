//! Build Config

/// The Database URI
pub const DATABASE_URL: &str = match option_env!("DATABASE_URL") {
    Some(val) => val,
    None => "sqlite3:///:memory:",
};
