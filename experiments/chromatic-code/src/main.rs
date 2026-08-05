//! # Chromatic Code 🌈🔐
//!
//! > "The best encryption is invisible. Steganography doesn't just protect data—it denies its existence."
//!
//! **Chromatic Code** is a steganography tool that hides source code (or any text) inside procedurally generated "Plasma" art. The image acts as a visual cipher: to the naked eye, it's just a psychedelic gradient. To the decoder, it's a carrier signal.
//!
//! ## Features
//!
//! -   **Procedural Cover Generation**: Creates unique "Plasma" images on the fly. No need for an existing image to hide things in.
//! -   **LSB Steganography**: Embeds data into the Least Significant Bits of the RGB channels.
//! -   **TUI Visualization**: A "Matrix-style" viewer that renders the image in the terminal (using half-blocks) and animates the decoding process in real-time.
//! -   **CLI Support**: Encode, Decode, and View commands.
//!
//! ## Usage
//!
//! ### Demo
//! Run the self-contained demo to see it in action (hides its own source code in a generated image):
//! ```bash
//! cargo run -p chromatic-code -- demo
//! ```
//!
//! ### Encode
//! Hide a text file inside a new image:
//! ```bash
//! cargo run -p chromatic-code -- encode -i src/main.rs -o secret.png --width 400 --height 200
//! ```
//!
//! ### View (TUI)
//! View and decode an image in the terminal:
//! ```bash
//! cargo run -p chromatic-code -- view -i secret.png
//! ```
//!
//! ### Decode (CLI)
//! Extract the hidden text to a file:
//! ```bash
//! cargo run -p chromatic-code -- decode -i secret.png -o extracted_code.rs
//! ```
//!
//! ## How it Works
//!
//! 1.  **Generation**: We generate a continuous plasma field using summed sine waves.
//! 2.  **Embedding**: We flatten the text into bits. We iterate through the image pixels and replace the LSB of each Red, Green, and Blue channel with our data bits.
//! 3.  **Visualization**: The TUI uses `ratatui` to render the image using `▀` (HalfBlock) characters, effectively giving us two pixels per character cell. The "decoding" animation is purely visual flair.
//!
//! ## Constraints
//!
//! -   Capacity is limited by image resolution (`Width * Height * 3` bits).
//! -   Requires `ratatui` compatible terminal.
//!
//! ---
//! *Built by Genesis ⚛️*
//!
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use std::io::Read;
use std::path::PathBuf;

mod steg;
mod tui;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Encodes text into a generated plasma image.
    Encode {
        /// Input text file to hide
        #[arg(short, long)]
        input: PathBuf,

        /// Output PNG file path
        #[arg(short, long)]
        output: PathBuf,

        /// Width of the generated image (default: 800)
        #[arg(long, default_value_t = 800)]
        width: u32,

        /// Height of the generated image (default: 600)
        #[arg(long, default_value_t = 600)]
        height: u32,
    },
    /// Decodes hidden text from an image.
    Decode {
        /// Input PNG file
        #[arg(short, long)]
        input: PathBuf,

        /// Output text file (optional, defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Views the image and decodes it in TUI.
    View {
        /// Input PNG file
        #[arg(short, long)]
        input: PathBuf,
    },
    /// Runs a self-contained demo.
    Demo,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Demo) {
        Commands::Encode {
            input,
            output,
            width,
            height,
        } => {
            let mut text = String::new();
            let limit = 1024 * 1024;
            let file = std::fs::File::open(&input).context("Failed to open input file")?;
            let bytes = file
                .take(limit + 1)
                .read_to_string(&mut text)
                .context("Failed to read input file")?;
            anyhow::ensure!(bytes <= limit as usize, "Input file exceeds 1MB limit");

            println!("Generating cover image ({}x{})...", width, height);
            let mut img = steg::generate_plasma(width, height);

            println!("Embedding payload ({} bytes)...", text.len());
            steg::embed(&mut img, text.as_bytes()).context("Failed to embed data")?;

            img.save(&output).context("Failed to save output image")?;
            println!("Saved to {:?}", output);
        }
        Commands::Decode { input, output } => {
            let img = image::open(&input)
                .context("Failed to open input image")?
                .to_rgb8();
            println!("Extracting data...");
            let data = steg::extract(&img).context("Failed to extract data")?;
            let text = String::from_utf8(data).context("Decoded data is not valid UTF-8")?;

            if let Some(out_path) = output {
                std::fs::write(&out_path, &text).context("Failed to write output file")?;
                println!("Decoded text saved to {:?}", out_path);
            } else {
                println!("--- Decoded Payload ---");
                println!("{}", text);
                println!("-----------------------");
            }
        }
        Commands::View { input } => {
            let img = image::open(&input)
                .context("Failed to open input image")?
                .to_rgb8();
            let data = steg::extract(&img).context("Failed to extract data")?;
            let text = String::from_utf8(data).context("Decoded data is not valid UTF-8")?;
            tui::run(img, text)?;
        }
        Commands::Demo => {
            println!("Running Demo...");
            // Use this source code as the payload
            let mut source_code = String::new();
            let limit = 1024 * 1024;
            let file_res = std::fs::File::open("experiments/chromatic-code/src/main.rs")
                .or_else(|_| std::fs::File::open("src/main.rs")); // Fallback if running from crate root

            if let Ok(file) = file_res {
                if let Ok(bytes) = file.take(limit + 1).read_to_string(&mut source_code) {
                    if bytes > limit as usize {
                        source_code = "Could not find source code, using dummy text.".to_string();
                    }
                } else {
                    source_code = "Could not find source code, using dummy text.".to_string();
                }
            } else {
                source_code = "Could not find source code, using dummy text.".to_string();
            }

            let mut img = steg::generate_plasma(200, 100); // Small size for TUI fit
            steg::embed(&mut img, source_code.as_bytes()).context("Failed to embed data")?;

            // We don't save to file in demo, just pass directly to TUI
            tui::run(img, source_code)?;
        }
    }

    Ok(())
}
