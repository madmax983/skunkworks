use clap::Parser;
use stego_attack::config::AttackConfig;
use stego_attack::stego;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "stego-generator")]
#[command(about = "Generates a Stego Attack Cartridge", long_about = None)]
struct Cli {
    /// Input image path
    #[arg(long)]
    input: PathBuf,

    /// Output image path
    #[arg(long)]
    output: PathBuf,

    /// Target X (0.0 - 1.0)
    #[arg(long, default_value_t = 0.5)]
    target_x: f32,

    /// Target Y (0.0 - 1.0)
    #[arg(long, default_value_t = 0.5)]
    target_y: f32,

    /// Agent Speed
    #[arg(long, default_value_t = 2.0)]
    speed: f32,

    /// Dissolve Rate
    #[arg(long, default_value_t = 0.01)]
    dissolve: f32,
}

fn main() {
    let cli = Cli::parse();

    println!("Loading input image: {:?}", cli.input);
    let mut img = image::open(&cli.input)
        .expect("Failed to open input image")
        .to_rgba8();

    let config = AttackConfig {
        target_x: cli.target_x,
        target_y: cli.target_y,
        agent_speed: cli.speed,
        dissolve_rate: cli.dissolve,
    };

    println!("Embedding config: {:?}", config);
    let json = serde_json::to_vec(&config).expect("Failed to serialize config");

    stego::embed(&mut img, &json).expect("Failed to embed data");

    println!("Saving output image: {:?}", cli.output);
    img.save(&cli.output).expect("Failed to save output image");

    println!("Done!");
}
