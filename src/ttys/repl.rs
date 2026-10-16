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

#[derive(Parser)]
#[command(override_usage="[COMMAND]", no_binary_name=true)]
pub struct ConfigRepl {
    #[command(subcommand)]
    pub command: Option<ConfigCommands>
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Connection timeout max seconds (max value 255)
    Timeout {
        seconds: u8
    },

    /// Toggle or set unverified HTTPS certs usage
    Insecure {
        use_unverified_https_certs: bool
    },

    /// Show current configuration
    Show,
    /// Exit the HTTP client configuration session
    Exit,
    /// Clear the terminal screen
    Clear,
}