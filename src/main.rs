use clap::{Parser, Subcommand};
use reqwest::Client;
use std::time::Duration;
mod engine;
mod ttys;

// Configures the main TTY / Repl handler
#[derive(Parser)]
#[command(name = "striker")]
#[command(about = "Ferramenta de Stress Test de Alta Performance", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<CliCommands>,
}

// Available commands enum
#[derive(Subcommand)]
enum CliCommands {
    Repl,
    Flood {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        requests: u32
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let http_client = Client::builder()
    .timeout(Duration::from_secs(10))
    .danger_accept_invalid_certs(true)
    .build()
    .expect("Error building HTTP Client");

    match &cli.command {
        Some(CliCommands::Flood {url, requests}) => {
            engine::flood_url(&http_client, &url, *requests).await;
        }
        Some(CliCommands::Repl) | None => {
            ttys::main_tty::main_tty(&http_client).await;
        }

    }

}

