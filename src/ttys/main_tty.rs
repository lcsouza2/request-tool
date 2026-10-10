use clap::Parser;
use reqwest::Client;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::engine;
use crate::ttys::repl::{Repl, ReplCommands};

pub async fn main_tty(http_client: &Client) {
    let mut rl = DefaultEditor::new().expect("Error initializing TTY");

    loop {
        let readline = rl.readline("striker> ");

        match readline {
            Ok(line) => {
                let cmd = line.trim();

                let parsed = Repl::try_parse_from(cmd.split_whitespace());

                if !cmd.is_empty() {
                    let _ = rl.add_history_entry(cmd);
                }

                match parsed {
                    Ok(repl) => match repl.command {
                        Some(ReplCommands::Flood { url, requests }) => {
                            engine::flood_url(http_client, &url, requests).await;
                        }
                        Some(ReplCommands::Config) => {
                            println!("Configuration mode is not implemented yet");
                        }
                        Some(ReplCommands::Clear) => {
                            println!("\x1B[2J\x1B[H");
                        }
                        Some(ReplCommands::Exit) => {
                            break;
                        }
                        None => {}
                    },
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
