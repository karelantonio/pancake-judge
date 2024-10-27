use actix_web::{App, HttpServer};
use clap::Parser;

/// The command line args
#[derive(Parser)]
#[command(version, about, name = "judge", bin_name = "judge")]
pub struct Cmd {
    #[cfg(unix)]
    #[arg(
        short = 'u',
        long = "unix",
        help = "Specify the UNIX Domain Socket where to listen for connections"
    )]
    unix: bool,
    #[arg(help = "The host, by default: 0.0.0.0:8000, or default.sock (when -u/--unix specified)")]
    host: Option<String>,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Load the .env vars
    let _ = dotenvy::dotenv()?;

    // Setup the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Parse the args
    let args = Cmd::parse();

    // Setup the server and delegate the setup
    let serv = HttpServer::new(|| App::new().configure(judge::config));
    let serv = if cfg!(unix) && args.unix {
        serv.bind_uds(args.host.unwrap_or("default.socket".to_string()))
    } else {
        serv.bind(args.host.unwrap_or("0.0.0.0:8000".to_string()))
    }?;

    serv.run().await?;

    Ok(())
}
