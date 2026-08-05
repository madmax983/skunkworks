//! # Hidden Brush 🐢🎨
//!
//! > "The best encryption is invisible."
//!
//! **Hidden Brush** is a steganographic virtual machine. It allows you to hide "Turtle Graphics" programs inside innocent-looking images. When executed, the hidden program takes control of the brush and draws a new image or animation.
//!
//! ## Concept
//!
//! We combine **LSB Steganography** with **Code Distribution**.
//! Instead of hiding a static message, we hide a *program*.
//! The image is the cartridge. The tool is the console.
//!
//! ## Usage
//!
//! ### Encode
//! Hide a script in a cover image (or generate noise):
//! ```bash
//! cargo run -- encode --script demo.asm --output secret.png
//! ```
//!
//! ### Run
//! Execute the hidden program:
//! ```bash
//! cargo run -- run --image secret.png
//! ```
//!
//! ## Bytecode
//! The VM supports a simple Turtle instruction set:
//! - `FWD`: Move forward
//! - `ROT <deg>`: Rotate
//! - `PEN <0|1>`: Pen up/down
//! - `COL <r> <g> <b>`: Set color
//! - `REP <count> <len>`: Repeat next `len` instructions `count` times.
//! - `SET_STEP <val>`: Set step size.
//! - `ADD_STEP <val>`: Modify step size (useful for spirals).
//!
//! ## Example Script
//! ```asm
//! COL 0 255 255
//! SET_STEP 2
//! REP 100 3
//! FWD
//! ROT 89
//! ADD_STEP 1
//! END
//! ```
//!
mod bytecode;
mod stego;
mod ui;
mod vm;

use anyhow::Result;
use clap::{Parser, Subcommand};
use image::{DynamicImage, Rgb, RgbImage};
use rand::Rng;
use std::fs;
use std::io::Read;
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
            let file = fs::File::open(&script)?;
            let mut script_content = String::new();
            let limit = 1024 * 1024; // 1MB limit
            let bytes_read = file.take(limit + 1).read_to_string(&mut script_content)?;

            if bytes_read as u64 > limit {
                anyhow::bail!("Script file {:?} exceeds 1MB limit", script);
            }
            let bytecode = bytecode::parse(&script_content)?;

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

            let instructions = bytecode::disassemble(&bytecode)?;
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
