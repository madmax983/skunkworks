use clap::{Parser, Subcommand};
use colored::*;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "quipu")]
#[command(about = "The Ancient Incan Data Serializer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encodes a JSON file into Quipu Artifact
    Encode {
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },
    /// Decodes a Quipu Artifact into JSON
    Decode {
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { input } => {
            let content = fs::read_to_string(input)?;
            let value: Value = serde_json::from_str(&content)?;
            let quipu_str = serde_quipu::to_string(&value)?;

            // Colorize output
            for line in quipu_str.lines() {
                if line.contains("[") && line.contains("]") {
                    // Primitive highlighting
                    let parts: Vec<&str> = line.split('[').collect();
                    print!("{}", parts[0]);
                    for p in parts.iter().skip(1) {
                        if let Some(end) = p.find(']') {
                            let color_name = &p[..end];
                            let rest = &p[end + 1..];
                            print!("[{}]", color_name.yellow().bold());
                            print!("{}", rest.cyan());
                        } else {
                            print!("[{}", p);
                        }
                    }
                    println!();
                } else {
                    println!("{}", line.green());
                }
            }
        }
        Commands::Decode { input } => {
            let content = fs::read_to_string(input)?;
            let value: Value = serde_quipu::from_str(&content)?;
            let json_str = serde_json::to_string_pretty(&value)?;
            println!("{}", json_str);
        }
    }

    Ok(())
}
