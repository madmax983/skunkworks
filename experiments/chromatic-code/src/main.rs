use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
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
            let text = fs::read_to_string(&input).context("Failed to read input file")?;
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
                fs::write(&out_path, &text).context("Failed to write output file")?;
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
            let source_code = fs::read_to_string("experiments/chromatic-code/src/main.rs")
                .or_else(|_| fs::read_to_string("src/main.rs")) // Fallback if running from crate root
                .unwrap_or_else(|_| "Could not find source code, using dummy text.".to_string());

            let mut img = steg::generate_plasma(200, 100); // Small size for TUI fit
            steg::embed(&mut img, source_code.as_bytes()).context("Failed to embed data")?;

            // We don't save to file in demo, just pass directly to TUI
            tui::run(img, source_code)?;
        }
    }

    Ok(())
}
