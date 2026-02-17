use clap::Parser;
use std::fs;
use std::path::PathBuf;
use stego_cartridge::{asm, stego};

#[derive(Parser)]
#[command(name = "pack")]
#[command(about = "Pack assembly code into a PNG image", long_about = None)]
struct Cli {
    /// Source assembly file
    source: PathBuf,
    /// Output PNG file
    output: PathBuf,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    println!("Packing {} into {}...", cli.source.display(), cli.output.display());

    let source_code = fs::read_to_string(&cli.source).map_err(|e| e.to_string())?;
    let bytecode = asm::assemble(&source_code)?;

    println!("Bytecode size: {} bytes", bytecode.len());

    // Generate cover
    let width = 256;
    let height = 256;
    let mut image = stego::generate_cover(&bytecode, width, height);

    // Embed
    stego::embed(&mut image, &bytecode)?;

    // Save
    image.save(&cli.output).map_err(|e| e.to_string())?;

    println!("Success!");
    Ok(())
}
