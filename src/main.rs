mod auth;
mod config;
mod http;

use clap::{Parser, Subcommand};

/// Read-only command line client for the Trading 212 public API.
#[derive(Parser)]
#[command(name = "t212", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Store your Trading 212 API credentials
    Auth,
    /// Remove stored credentials
    Deauth,
    /// Check Trading 212 account connection status
    Status,
    /// Fetch basic portfolio overview stats
    Overview,
    /// compute more basic portfilio stats
    Analytics,
    /// Show account cash balance
    Cash,
    /// List open positions
    Positions,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Auth => auth::run_auth()?,
        Command::Deauth => auth::run_deauth()?,
        Command::Status => {
            let config = config::Config::load(config::Environment::Live)?;
            let client = http::Client::new(config);
            match client.check_connection() {
                Ok(()) => println!("Connected to Trading 212. Credentials are valid."),
                Err(e) => {
                    eprintln!("Not connected: {e}");
                    std::process::exit(1);
                }
            }
        }
        Command::Overview => {
            let config = config::Config::load(config::Environment::Live)?;
            let client = http::Client::new(config);

            let body = client.get("/equity/account/summary")?;

            // TODO: Format JSON to typed output and clean cli
            let parsed: serde_json::Value = serde_json::from_str(&body)?;
            println!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        Command::Cash => {
            let config = config::Config::load(config::Environment::Live)?;
            let client = http::Client::new(config);

            let body = client.get("/equity/account/cash")?;

            // TODO: Format JSON to typed output and clean cli
            let parsed: serde_json::Value = serde_json::from_str(&body)?;
            println!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        Command::Positions => {
            let config = config::Config::load(config::Environment::Live)?;
            let client = http::Client::new(config);

            let body = client.get("/equity/positions")?;

            // TODO: Format JSON to typed output and clean cli
            let parsed: serde_json::Value = serde_json::from_str(&body)?;
            println!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        Command::Analytics => println!("Analytics: not implemented yet"),
    }
    Ok(())
}