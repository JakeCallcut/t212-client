mod auth;
mod config;
mod http;

use owo_colors::OwoColorize;
use clap::{Parser, Subcommand};

/// Read-only command line client for the Trading 212 public API.
#[derive(Parser)]
#[command(name = "t212", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
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
        None => {
            banner();
            println!("Run {} to see everything you can do or run {} to log in", "t212 help".bold(), "t212 auth".bold());
        }
        Some(Command::Auth) => auth::run_auth()?,
        Some(Command::Deauth) => auth::run_deauth()?,
        Some(Command::Status) => {
            let config = config::Config::load(config::Environment::Demo)?;
            let client = http::Client::new(config);
            match client.check_connection() {
                Ok(()) => println!("{}", "Connected to Trading 212. Credentials are valid.".green().bold()),
                Err(e) => {
                    eprintln!("{}: {e}","Not connected".red().bold());
                    std::process::exit(1);
                }
            }
        }
        Some(Command::Overview) => {
            let config = config::Config::load(config::Environment::Live)?;
            let client = http::Client::new(config);

            let body = client.get("/equity/account/summary")?;

            // TODO: Format JSON to typed output and clean cli
            let parsed: serde_json::Value = serde_json::from_str(&body)?;
            println!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        Some(Command::Cash) => {
            let config = config::Config::load(config::Environment::Live)?;
            let client = http::Client::new(config);

            let body = client.get("/equity/account/cash")?;

            // TODO: Format JSON to typed output and clean cli
            let parsed: serde_json::Value = serde_json::from_str(&body)?;
            println!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        Some(Command::Positions) => {
            let config = config::Config::load(config::Environment::Live)?;
            let client = http::Client::new(config);

            let body = client.get("/equity/positions")?;

            // TODO: Format JSON to typed output and clean cli
            let parsed: serde_json::Value = serde_json::from_str(&body)?;
            println!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        Some(Command::Analytics) => println!("Analytics: not implemented yet"),
    }
    Ok(())
}

fn banner() {
    let art = r#"
      _________  ______      ________    ___________   ________
     /_  __/__ \<  /__ \    / ____/ /   /  _/ ____/ | / /_  __/
      / /  __/ // /__/ /   / /   / /    / // __/ /  |/ / / /
     / /  / __// // __/   / /___/ /____/ // /___/ /|  / / /
    /_/  /____/_//____/   \____/_____/___/_____/_/ |_/ /_/
    "#;
    println!("{}", art.blue().bold());
    println!("{}", "A read-only Trading 212 API client by Jake".blue().dimmed());
    println!();
}