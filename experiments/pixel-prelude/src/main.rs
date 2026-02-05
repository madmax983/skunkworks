use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use std::fs;

mod stego;
mod tui;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Hide a file inside an image (Use PNG/BMP for lossless!)
    Hide {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        payload: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
    /// Reveal a file hidden inside an image
    Reveal {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
    /// Inspect the image for hidden data (Visual Mode)
    Inspect {
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hide { input, payload, output } => {
            let mut img = image::open(input)?;
            let data = fs::read(payload)?;
            println!("Encoding {} bytes from {:?} into {:?}...", data.len(), payload, input);
            stego::embed_bytes(&mut img, &data)?;
            img.save(output)?;
            println!("Saved to {:?}", output);
        }
        Commands::Reveal { input, output } => {
            let img = image::open(input)?;
            println!("Decoding from {:?}...", input);
            let data = stego::extract_bytes(&img)?;
            fs::write(output, &data)?;
            println!("Extracted {} bytes to {:?}", data.len(), output);
        }
        Commands::Inspect { input } => {
            let img = image::open(input)?;
            tui::run(img)?;
        }
    }

    Ok(())
}
