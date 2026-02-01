use clap::Parser;
use gallifrey_db::GallifreyStore;
use std::io::{self, Write};
use chrono::DateTime;

#[derive(Parser)]
#[command(name = "gallifrey")]
#[command(about = "Time-Travel Key-Value Store REPL", long_about = None)]
struct Cli {}

fn main() -> anyhow::Result<()> {
    // We parse args just to check for help/version, otherwise we drop into REPL
    if std::env::args().len() > 1 {
       let _ = Cli::parse();
    }

    let mut store = GallifreyStore::new();

    println!("🔭 GallifreyDB v0.1.0");
    println!("Type 'help' for commands.");

    loop {
        print!("gallifrey> ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];

        match command {
            "exit" | "quit" => break,
            "put" => {
                if parts.len() < 3 {
                    println!("Usage: put <key> <value>");
                } else {
                    let key = parts[1].to_string();
                    let value = parts[2..].join(" ");
                    store.put(key, value);
                    println!("OK");
                }
            }
            "get" => {
                if parts.len() < 2 {
                    println!("Usage: get <key>");
                } else {
                    match store.get(parts[1]) {
                        Some(val) => println!("{}", val),
                        None => println!("(nil)"),
                    }
                }
            }
            "history" => {
                if parts.len() < 2 {
                    println!("Usage: history <key>");
                } else {
                    match store.history(parts[1]) {
                        Some(history) => {
                            for (ts, val) in history {
                                println!("[{}] {}", ts.to_rfc3339(), val);
                            }
                        }
                        None => println!("(nil)"),
                    }
                }
            }
            "get_at" => {
                 if parts.len() < 3 {
                    println!("Usage: get_at <key> <iso-8601-timestamp>");
                } else {
                    let key = parts[1];
                    let ts_str = parts[2];
                    match DateTime::parse_from_rfc3339(ts_str) {
                        Ok(ts) => {
                             match store.get_at(key, ts.into()) {
                                Some(val) => println!("{}", val),
                                None => println!("(nil)"),
                            }
                        }
                        Err(e) => println!("Invalid timestamp: {}", e),
                    }
                }
            }
            "help" => {
                println!("Commands:");
                println!("  put <key> <value>             Set a value");
                println!("  get <key>                     Get latest value");
                println!("  history <key>                 Show all versions");
                println!("  get_at <key> <timestamp>      Time travel query (ISO-8601)");
                println!("  exit                          Exit");
            }
            _ => println!("Unknown command. Type 'help'."),
        }
    }

    Ok(())
}
