use actix_web::{App, HttpServer};
use clap::Parser;
use judge::build_config;
use judge_db::DatabaseService;
use log::{debug, info};
use thiserror::Error;

/// The possible errors
#[derive(Error, Debug)]
pub enum JudgeError {
    #[error("Could not load .env file, try creating it via `touch .env` and add the necessary data (See the README)")]
    DotEnvy(#[source] dotenvy::Error),
    #[error("Error connecting to the database")]
    Db(#[source] judge_db::ConnectError),
    #[error("Error migrating database")]
    Migrate(#[source] judge_db::MigrateError),
    #[error("Could not bind to address")]
    Bind(#[source] std::io::Error),
    #[error("Error occurred in the web framework")]
    Running(#[source] std::io::Error),
}

/// The command line args
#[derive(Parser)]
#[command(
    version,
    about,
    name = "judge",
    bin_name = "judge",
    about = "The judge CLI"
)]
pub enum Cmd {
    #[command(name = "run", about = "Run the server")]
    Run {
        #[arg(
            short = 'u',
            long = "unix",
            help = "Specify the UNIX Domain Socket where to listen for connections"
        )]
        unix: bool,
        #[arg(
            help = "The host, by default: 0.0.0.0:8000, or default.sock (when -u/--unix specified)"
        )]
        host: Option<String>,
    },
    #[command(
        name = "migrate",
        about = "Run the necessary migrations in the database"
    )]
    Migrate {},
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Load the .env vars
    let _ = dotenvy::dotenv().map_err(JudgeError::DotEnvy)?;

    // Setup the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Connect to the DB
    info!("Connecting to the database");
    DatabaseService::setup_drivers();
    let db = DatabaseService::new(build_config::DATABASE_URL)
        .await
        .map_err(JudgeError::Db)?;

    // Parse the args
    match Cmd::parse() {
        Cmd::Run { unix, host } => {
            // Setup the server and delegate the setup
            let serv = HttpServer::new(|| App::new().configure(judge::config));
            let serv = if unix {
                let addr = host.unwrap_or("default.socket".to_string());
                debug!("Bind (unix): {addr}");
                serv.bind_uds(addr)
            } else {
                let addr = host.unwrap_or("0.0.0.0:8000".to_string());
                debug!("Bind: {addr}");
                serv.bind(addr)
            }
            .map_err(JudgeError::Bind)?;

            serv.run().await.map_err(JudgeError::Running)?;
        }
        Cmd::Migrate {} => {
            // Run the migrations
            info!("Running migrations...");
            db.run_migrations().await.map_err(JudgeError::Migrate)?;
            info!("Done");
        }
    };

    Ok(())
}
