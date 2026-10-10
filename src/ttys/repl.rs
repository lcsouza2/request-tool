use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(override_usage="[COMMAND]", no_binary_name=true)]
pub struct Repl {
    #[command(subcommand)]
    pub command: Option<ReplCommands>
}

#[derive(Subcommand)]
pub enum ReplCommands {
    /// Enter HTTP client configuration mode
    Config,

    /// Flood requests to a given target
    Flood {
        /// Target URL to flood
        url: String,
        /// Number of requests
        requests: u32,
    },

    /// Exit the REPL session
    Exit,

    /// Clear the terminal screen
    Clear,
}

