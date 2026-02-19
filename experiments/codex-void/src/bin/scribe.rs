use anyhow::{Context, Result};
use clap::Parser;
use codex_void::starmap::StarMap;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Text to encode
    #[arg(value_name = "TEXT")]
    text: Option<String>,

    /// Input file to encode
    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,

    /// Output image file
    #[arg(short, long, value_name = "FILE", default_value = "codex.png")]
    output: PathBuf,

    /// Width in glyphs (default: auto)
    #[arg(long)]
    width: Option<u32>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let data = if let Some(path) = cli.input {
        fs::read(path).context("Failed to read input file")?
    } else if let Some(text) = cli.text {
        text.into_bytes()
    } else {
        anyhow::bail!("Must provide either TEXT or --input FILE");
    };

    if data.is_empty() {
        anyhow::bail!("Input data is empty");
    }

    // Determine width
    let width = cli.width.unwrap_or_else(|| {
        let len = data.len() as f64;
        let w = len.sqrt().ceil() as u32;
        // Make it at least a bit wide
        if w < 10 {
            w.max(1)
        } else {
            w
        }
    });

    println!("Encoding {} bytes into grid width {}...", data.len(), width);

    let starmap = StarMap::new(&data, width);
    let img = starmap.generate();

    img.save(&cli.output)
        .context("Failed to save output image")?;

    println!("Saved star map to {}", cli.output.display());

    Ok(())
}
