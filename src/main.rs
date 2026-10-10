use clap::Parser;
use reqwest::Client;
use std::time::Duration;
mod engine;
mod ttys;
mod cli;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    let http_client = Client::builder()
    .timeout(Duration::from_secs(10))
    .danger_accept_invalid_certs(true)
    .build()
    .expect("Error building HTTP Client");

    match &cli.command {
        Some(cli::CliCommands::Flood {url, requests}) => {
            engine::flood_url(&http_client, &url, *requests).await;
        }
        Some(cli::CliCommands::Repl) | None => {
            ttys::main_tty::main_tty(&http_client).await;
        }

    }

}

