use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use constellation_cipher::{encode, decode};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode a file into a Constellation Image
    Encode {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output PNG file path
        #[arg(short, long)]
        output: PathBuf,

        /// Secret key for steganography
        #[arg(short, long)]
        key: String,

        /// Width of the output image
        #[arg(long, default_value_t = 1024)]
        width: u32,

        /// Height of the output image
        #[arg(long, default_value_t = 1024)]
        height: u32,
    },
    /// Decode a Constellation Image back to a file
    Decode {
        /// Input PNG file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Secret key
        #[arg(short, long)]
        key: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { input, output, key, width, height } => {
            let data = fs::read(&input).with_context(|| format!("Failed to read input file {:?}", input))?;
            println!("Encoding {} bytes...", data.len());
            let img = encode(&data, &key, width, height)?;
            img.save(&output).with_context(|| format!("Failed to save image {:?}", output))?;
            println!("Encoded successfully to {:?}", output);
        }
        Commands::Decode { input, output, key } => {
            let img = image::open(&input).with_context(|| format!("Failed to open image {:?}", input))?.to_rgba8();
            println!("Decoding...");
            let data = decode(&img, &key)?;
            fs::write(&output, &data).with_context(|| format!("Failed to write output file {:?}", output))?;
            println!("Decoded {} bytes successfully to {:?}", data.len(), output);
        }
    }
    Ok(())
}
