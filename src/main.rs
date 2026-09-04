mod auth;
mod config;
mod http;
mod models;
mod utils;

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
            let summary: models::AccountSummary = serde_json::from_str(&body)?;
            print_overview(&summary);
        }
        Some(Command::Cash) => {
            let config = config::Config::load(config::Environment::Live)?;
            let client = http::Client::new(config);

            let body = client.get("/equity/account/summary")?;

            // TODO: Format JSON to typed output and clean cli
            let cash: models::AccountSummary = serde_json::from_str(&body)?;
            print_cash(&cash);
        }
        Some(Command::Positions) => {
            let config = config::Config::load(config::Environment::Live)?;
            let client = http::Client::new(config);

            let body = client.get("/equity/positions")?;
            let mut positions: Vec<models::Position> = serde_json::from_str(&body)?;

            // Sort by current value, largest first.
            positions.sort_by(|a, b| {
                b.wallet_impact
                    .current_value
                    .partial_cmp(&a.wallet_impact.current_value)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            print_positions(&positions);
        }
        Some(Command::Analytics) => println!("Analytics: not implemented yet"),
    }
    Ok(())
}

    ///formats welcome banner
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

    ///print account summary formatted
fn print_overview(s: &models::AccountSummary) {
    let c = &s.currency;

    println!();
    println!("{}", "Account overview".blue().bold());
    println!("  {:<18} {} ({})", "Account".dimmed(), s.id, c);
    println!("  {:<18} {:>12.2} {c}", "Total value".dimmed(), s.total_value);
    println!();

    println!("{}", "Cash".blue().bold());
    println!("  {:<18} {:>12.2} {c}", "Total Uninvested".dimmed(), (s.cash.available_to_trade + s.cash.in_pies + s.cash.reserved_for_orders));
    println!();

    println!("{}", "Investments".blue().bold());
    println!("  {:<18} {:>12.2} {c}", "Current value".dimmed(), s.investments.current_value);
    println!("  {:<18} {:>12.2} {c}", "Total cost".dimmed(), s.investments.total_cost);
    print_pl("Unrealised P&L", s.investments.unrealized_profit_loss, s.investments.total_cost, c);
    print_pl("Realised P&L", s.investments.realized_profit_loss, s.investments.total_cost, c);
    println!();
}

    ///compute and format PNL statistic
fn print_pl(label: &str, value: f64, cost: f64, currency: &str) {
    //avoid dividing by 0
    let pct = if cost != 0.0 { value / cost * 100.0 } else { 0.0 };

    //format in currency
    let text = format!("{value:>12.2} {currency}  ({pct:+.2}%)");

    //choose colour depending on value
    let coloured = if value >= 0.0 {
        text.green().to_string()
    } else {
        text.red().to_string()
    };
    println!("  {:<18} {}", label.dimmed(), coloured);
}

///format and print cash allocations
fn print_cash(s: &models::AccountSummary) {
    let c = &s.currency;

    println!();
    println!("{}", "Cash Allocation".blue().bold());
    println!("  {:<18} {:>12.2} {c}", "Total Uninvested".dimmed(), (s.cash.available_to_trade + s.cash.in_pies + s.cash.reserved_for_orders));
    println!("  {:<18} {:>12.2} {c}", "Available".dimmed(), (s.cash.available_to_trade));
    println!("  {:<18} {:>12.2} {c}", "in Pies".dimmed(), (s.cash.in_pies));
    println!("  {:<18} {:>12.2} {c}", "Reserved".dimmed(), (s.cash.reserved_for_orders));
    println!();
}

///format and print table of open positions
fn print_positions(positions: &[models::Position]) {
    if positions.is_empty() {
        println!("{}", "No open positions.".dimmed());
        return;
    }

    // Wallet currency is your account currency; consistent across rows.
    let wallet_ccy = &positions[0].wallet_impact.currency;

    println!();
    println!("{} {}", "Open positions".blue().bold(), "(by descending value)".blue().dimmed());
    println!(
        "  {:<30} {:>10} {:>14} {:>14} {:>20}",
        "Instrument".dimmed(),
        "Qty".dimmed(),
        "Value".dimmed(),
        "Cost".dimmed(),
        "P&L".dimmed(),
    );

    let mut total_value = 0.0;
    let mut total_cost = 0.0;

    for p in positions {
        let w = &p.wallet_impact;
        total_value += w.current_value;
        total_cost += w.total_cost;

        let pct = if w.total_cost != 0.0 {
            w.unrealized_profit_loss / w.total_cost * 100.0
        } else {
            0.0
        };
        let pl = format!("{:>10.2} ({:+.1}%)", w.unrealized_profit_loss, pct);
        let pl = if w.unrealized_profit_loss >= 0.0 {
            pl.green().to_string()
        } else {
            pl.red().to_string()
        };

        println!(
            "  {:<30} {:>10.4} {:>14.2} {:>14.2} {:>30}",
            crate::utils::truncate(&p.instrument.name, 30), p.quantity, w.current_value, w.total_cost, pl,
        );
    }

    let total_pl = total_value - total_cost;
    let total_pct = if total_cost != 0.0 { total_pl / total_cost * 100.0 } else { 0.0 };
    let total_line = format!("{total_pl:>10.2} ({total_pct:+.1}%)");
    let total_line = if total_pl >= 0.0 { total_line.green().to_string() } else { total_line.red().to_string() };

    println!(
        "  {:<30} {:>10} {:>14.2} {:>14.2} {:>30}",
        "TOTAL".bold(),
        "",
        total_value.bold(),
        total_cost.bold(),
        total_line.bold(),
    );
    println!("  {}", format!("all values in {wallet_ccy}").dimmed());
    println!();
}

