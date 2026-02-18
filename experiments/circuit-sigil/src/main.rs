mod circuit;
mod stego;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use image::ImageReader;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "circuit-sigil")]
#[command(about = "Git Commit Verification via Procedural PCB Art")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a Circuit Sigil from a commit hash
    Generate {
        /// The commit hash (or any string seed)
        #[arg(value_name = "HASH")]
        hash: String,

        /// The author name to embed
        #[arg(short, long)]
        author: String,

        /// The commit message to embed
        #[arg(short, long)]
        message: String,

        /// Output image path
        #[arg(short, long, default_value = "sigil.png")]
        output: PathBuf,
    },
    /// Verify a Circuit Sigil against a hash
    Verify {
        /// The sigil image path
        #[arg(value_name = "IMAGE")]
        image: PathBuf,

        /// The expected commit hash
        #[arg(value_name = "HASH")]
        hash: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            hash,
            author,
            message,
            output,
        } => {
            println!("Generating Sigil for hash: {}", hash);
            let generator = circuit::CircuitGenerator::new(512, 512);
            let (mut img, pads) = generator.generate(&hash);

            let payload = format!("Author: {}\nMessage: {}", author, message);
            println!("Embedding payload: {:?}", payload);

            stego::embed(&mut img, &payload, &pads).context("Failed to embed data into Sigil")?;

            img.save(&output).context("Failed to save output image")?;

            println!("Sigil generated successfully: {:?}", output);
        }
        Commands::Verify { image, hash } => {
            println!("Verifying Sigil: {:?} against hash: {}", image, hash);

            // 1. Load Image
            let loaded_img = ImageReader::open(&image)?
                .decode()
                .context("Failed to decode image")?
                .to_rgba8();

            // 2. Generate Expected Layout
            let generator = circuit::CircuitGenerator::new(512, 512);
            let (expected_img, pads) = generator.generate(&hash);

            // 3. Visual Verification
            if loaded_img.width() != expected_img.width()
                || loaded_img.height() != expected_img.height()
            {
                println!("❌ Visual Verification FAILED: Dimensions mismatch.");
            } else {
                let mut diff_pixels = 0;
                for y in 0..loaded_img.height() {
                    for x in 0..loaded_img.width() {
                        let p1 = loaded_img.get_pixel(x, y);
                        let p2 = expected_img.get_pixel(x, y);

                        // Compare ignoring LSB (mask with 0xFE)
                        let mut match_pixel = true;
                        for c in 0..3 {
                            // RGB only, ignore Alpha for now? No, alpha should match too.
                            if (p1[c] & 0xFE) != (p2[c] & 0xFE) {
                                match_pixel = false;
                                break;
                            }
                        }
                        // Alpha check (usually 255)
                        if p1[3] != p2[3] {
                            match_pixel = false;
                        }

                        if !match_pixel {
                            diff_pixels += 1;
                        }
                    }
                }

                if diff_pixels == 0 {
                    println!("✅ Visual Verification SUCCESS: Image matches hash perfectly (ignoring LSBs).");
                } else {
                    println!(
                        "❌ Visual Verification FAILED: {} pixels differ visually.",
                        diff_pixels
                    );
                }
            }

            // 4. Extract Payload
            match stego::extract(&loaded_img, &pads) {
                Ok(payload) => {
                    println!("✅ Steganography Extraction SUCCESS:");
                    println!("--- BEGIN PAYLOAD ---");
                    println!("{}", payload);
                    println!("--- END PAYLOAD ---");
                }
                Err(e) => {
                    println!("❌ Steganography Extraction FAILED: {}", e);
                }
            }
        }
    }

    Ok(())
}
