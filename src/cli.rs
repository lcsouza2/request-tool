use clap::{Parser, Subcommand};

// Configures the main CLI handler
#[derive(Parser)]
#[command(name = "striker")]
#[command(about = "High-performance stress testing tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<CliCommands>,
}

// Available commands enum
#[derive(Subcommand)]
pub enum CliCommands {
    /// Start the interactive REPL session
    Repl,
    /// Flood requests to a given target
    Flood {
        #[arg(short, long)]
        /// Target URL to flood
        url: String,
        #[arg(short, long)]
        /// Number of requests
        requests: u32
    },
}
