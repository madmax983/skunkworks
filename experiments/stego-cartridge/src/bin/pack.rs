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

    println!(
        "Packing {} into {}...",
        cli.source.display(),
        cli.output.display()
    );

    let limit = 10 * 1024 * 1024;
    let mut file = std::fs::File::open(&cli.source).map_err(|e| e.to_string())?;
    let mut source_code = String::new();
    use std::io::Read;
    let bytes_read = file
        .take(limit + 1)
        .read_to_string(&mut source_code)
        .map_err(|e| e.to_string())?;
    if bytes_read > limit as usize {
        return Err("Source file is too large! Maximum allowed size is 10MB.".to_string());
    }

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
