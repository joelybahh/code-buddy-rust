use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Commit changes to the repository
    Commit(CommitArgs),
}

#[derive(Args, Debug)]
pub struct CommitArgs {
    /// The commit message or reason for the change
    #[arg(short = 'r', long)]
    reason: Option<String>,

    /// The type of change (e.g., feat, fix, docs)
    #[arg(short = 't', long)]
    r#type: Option<String>,

    /// The scope of the change
    #[arg(short = 's', long)]
    scope: Option<String>,

    /// The issue number related to this commit
    #[arg(short = 'i', long)]
    issue: Option<String>,

    /// Indicates if this is a breaking change
    #[arg(short = 'b', long)]
    breaking: bool,
}
