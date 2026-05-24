//! A utility to generate noise images to act as covers or payloads for the simulation.

use clap::Parser;
use image::{Rgba, RgbaImage};
use std::path::PathBuf;

/// Command line arguments for generating a noise image.
#[derive(Parser)]
struct Cli {
    /// The output path where the noise image will be saved.
    #[arg(long, default_value = "experiments/stego-attack/assets/input.png")]
    output: PathBuf,
}

/// The main entry point for the noise generator utility.
fn main() {
    let cli = Cli::parse();
    let width = 500;
    let height = 500;
    let mut img = RgbaImage::new(width, height);
    for pixel in img.pixels_mut() {
        if rand::random::<f32>() > 0.9 {
            *pixel = Rgba([rand::random(), rand::random(), rand::random(), 255]);
        } else {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }
    img.save(&cli.output).expect("Failed to save image");
    println!("Generated noise image at {:?}", cli.output);
}
