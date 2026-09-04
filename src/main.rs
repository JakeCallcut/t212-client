use clap::{Parser, Subcommand};

/// Read-only command line client for the Trading 212 public API
#[derive(Parser)]
#[command(name = "t212", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show account cash balance
    Cash,
    /// List open positions
    Positions,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Cash => println!("cash: not implemented yet"),
        Command::Positions => println!("positions: not implemented yet"),
    }
}