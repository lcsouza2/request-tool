use clap::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::engine;
use crate::ttys::repl::{ConfigRepl, ConfigCommands};

pub async fn config_tty(editor: &DefaultEditor) {

    loop {
        let readline = editor.readline("striker(config)> ");

        match readline {
            Ok(line) => {
                let cmd = line.trim();

                let parsed = ConfigRepl::try_parse_from(cmd.split_whitespace());

                if !cmd.is_empty() {
                    let _ = editor.add_history_entry(cmd);
                }

                match parsed {
                    Ok(repl) => match repl.command {
                        Some(ConfigCommands::Timeout { seconds }) => {
                            engine::flood_url().await;
                        }
                        Some(ConfigCommands::Insecure) => {
                            println!("Configuration mode is not implemented yet");
                        }
                        Some(ConfigCommands::Clear) => {
                            println!("\x1B[2J\x1B[H");
                        }
                        Some(ConfigCommands::Exit) => {
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
