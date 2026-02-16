mod rune;
mod stego;
mod vm;

use anyhow::{Result, Context};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use sha2::{Sha256, Digest};

#[derive(Parser)]
#[command(name = "wasm-runes")]
#[command(about = "Hide WASM in Steganographic Runes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Forge a Rune from a WASM binary
    Forge {
        #[arg(value_name = "WASM_FILE")]
        input: PathBuf,
        #[arg(value_name = "RUNE_IMAGE", default_value = "rune.png")]
        output: PathBuf,
    },
    /// Invoke a Rune (extract and run WASM)
    Invoke {
        #[arg(value_name = "RUNE_IMAGE")]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Forge { input, output } => {
            let wasm_bytes = std::fs::read(&input)
                .with_context(|| format!("Failed to read input file: {:?}", input))?;

            // Calculate hash for visual seed
            let mut hasher = Sha256::new();
            hasher.update(&wasm_bytes);
            let result = hasher.finalize();

            // Generate Rune
            // Estimate size based on payload
            // 3 bits per pixel -> 8 bits per byte
            // Pixels needed = (bytes * 8) / 3
            // Area = Pixels. Side = sqrt(Pixels)
            let needed_pixels = (wasm_bytes.len() as f64 * 8.0 / 3.0).ceil() as u32 + 1000; // + header buffer
            let side = (needed_pixels as f64).sqrt().ceil() as u32;
            let width = side.max(512); // Minimum size for aesthetics
            let height = width;

            println!("Forging Rune...");
            println!("Payload size: {} bytes", wasm_bytes.len());
            println!("Rune dimensions: {}x{}", width, height);

            let mut img = rune::generate(&result, width, height);

            // Embed
            stego::embed(&mut img, &wasm_bytes)?;

            img.save(&output)?;
            println!("Rune forged successfully: {:?}", output);
        }
        Commands::Invoke { input } => {
            println!("Reading Rune: {:?}", input);
            let img = image::open(&input)?.to_rgba8();

            println!("Extracting essence...");
            let wasm_bytes = stego::extract(&img)?;
            println!("Extracted {} bytes.", wasm_bytes.len());

            println!("Invoking...");
            vm::run(&wasm_bytes)?;
        }
    }
    Ok(())
}
