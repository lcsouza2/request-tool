use reqwest::Client;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::engine;

pub async fn main_tty(http_client: &Client) {
    let mut rl = DefaultEditor::new().expect("Error initializing TTY");

    loop {
        let readline = rl.readline("striker> ");

        match readline {
            Ok(line) => {
                let cmd = line.trim();

                if !cmd.is_empty() {
                    let _ = rl.add_history_entry(cmd);
                }

                match cmd {
                    "exit" => {
                        break;
                    }
                    "help" => {
                        println!("nada ainda");
                    }
                    "clear" => {
                        println!("\033[2J");
                        println!("\033[H");
                    }
                    cmd if cmd.starts_with("flood") => {
                        let parts: Vec<&str> = cmd.split_whitespace().collect();
                        let requests = parts[2].parse::<u32>();

                        match requests {
                            Ok(requests) => {
                                engine::flood_url(&http_client, parts[1], requests).await;
                            }
                            Err(err) => {
                                println!("Invalid requests {}", err);
                            }
                        }

                    }
                    _ => {println!("Unknown command");}
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
