mod cli;
mod commands;
mod config;

use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Commit(args) => commands::commit::run(args).await,
    }
}
