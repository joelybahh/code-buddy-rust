mod cli;
mod config;
mod commands;

use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Commit => commands::commit::run().await,
    }
}
