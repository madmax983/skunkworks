use image::{Rgba, RgbaImage};
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "experiments/stego-attack/assets/input.png")]
    output: PathBuf,
}

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
