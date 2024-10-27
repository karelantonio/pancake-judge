//! Build Config

/// The Database URI
pub const DATABASE_URL: &str = match option_env!("DATABASE_URL") {
    Some(val) => val,
    None => "sqlite3:///:memory:",
};

/// The submissions directory (where to store the input&output)
pub const SUBMISSIONS_DIR: &str = match option_env!("SUBMISSIONS_DIR") {
    Some(val) => val,
    None => "/submissions",
};

/// The static directory
pub const STATIC_DIR: &str = match option_env!("STATIC_DIR") {
    Some(val) => val,
    None => "/var/www/html",
};
