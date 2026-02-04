mod bytecode;
mod stego;
mod ui;
mod vm;

use anyhow::Result;
use clap::{Parser, Subcommand};
use image::{DynamicImage, Rgb, RgbImage};
use rand::Rng;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hidden-brush")]
#[command(about = "Steganographic Turtle Graphics VM", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Embeds a turtle script into an image
    Encode {
        /// Path to the script file (.asm)
        #[arg(short, long)]
        script: PathBuf,

        /// Path to the cover image (optional, generates noise if missing)
        #[arg(short, long)]
        image: Option<PathBuf>,

        /// Path to the output image (.png)
        #[arg(short, long)]
        output: PathBuf,

        /// Width of generated noise image
        #[arg(long, default_value_t = 500)]
        width: u32,

        /// Height of generated noise image
        #[arg(long, default_value_t = 500)]
        height: u32,
    },
    /// Runs a script hidden in an image
    Run {
        /// Path to the stego-image
        #[arg(short, long)]
        image: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            script,
            image,
            output,
            width,
            height,
        } => {
            let script_content = fs::read_to_string(&script)?;
            let bytecode = bytecode::Assembler::parse(&script_content)?;

            let cover = if let Some(path) = image {
                image::open(path)?
            } else {
                generate_noise(width, height)
            };

            let stego_image = stego::encode(&cover, &bytecode)?;
            stego_image.save(&output)?;
            println!("Encoded {} bytes into {}", bytecode.len(), output.display());
        }
        Commands::Run { image } => {
            let img = image::open(&image)?;
            let bytecode = stego::decode(&img)?;
            println!("Decoded {} bytes of bytecode.", bytecode.len());

            let instructions = bytecode::Assembler::disassemble(&bytecode)?;
            println!("Parsed {} instructions.", instructions.len());

            let mut turtle = vm::Turtle::new();
            turtle.run(&instructions)?;

            println!(
                "Executed. Path has {} segments. Starting UI...",
                turtle.path.len()
            );
            ui::run(turtle)?;
        }
    }
    Ok(())
}

fn generate_noise(width: u32, height: u32) -> DynamicImage {
    let mut rng = rand::thread_rng();
    let mut img = RgbImage::new(width, height);

    for pixel in img.pixels_mut() {
        let r = rng.gen();
        let g = rng.gen();
        let b = rng.gen();
        *pixel = Rgb([r, g, b]);
    }

    DynamicImage::ImageRgb8(img)
}
