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
        Command::Overview => println!("Overview: not implemented yet"),
        Command::Analytics => println!("Analytics: not implemented yet"),
        Command::Cash => println!("cash: not implemented yet"),
        Command::Positions => println!("positions: not implemented yet"),
    }
    Ok(())
}