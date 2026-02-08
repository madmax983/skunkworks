mod parser;
mod mapper;
mod tui;
mod audio_backend;

use anyhow::Result;
use clap::Parser;
use crossbeam_channel::bounded;
use std::path::PathBuf;
use tui::App;
use audio_backend::AudioSystem;
use resonance_audio::audio::AudioCommand;
use resonance_audio::physics::PhysicsGrid; // Needed for temp grid

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Rust file to resonate
    #[arg(short, long)]
    file: PathBuf,

    /// Grid width (default 128)
    #[arg(long, default_value_t = 128)]
    width: usize,

    /// Grid height (default 128)
    #[arg(long, default_value_t = 128)]
    height: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Parse File
    println!("Parsing {:?}...", args.file);
    let entities = parser::parse_file(&args.file)?;
    let lines = parser::read_lines(&args.file)?;

    // 2. Setup Audio System / Simulation
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    let mut audio_system = AudioSystem::new(args.width, args.height, cmd_rx, snap_tx)?;

    // 3. Map Code to Physics
    // Create a temporary grid to calculate walls
    let mut temp_grid = PhysicsGrid::new(args.width, args.height);
    mapper::map_to_grid(&entities, &lines, &mut temp_grid);

    // Send wall commands
    for y in 0..args.height {
        for x in 0..args.width {
            let idx = y * args.width + x;
            // Check bounds just in case, though temp_grid matches args
            if idx < temp_grid.walls.len() && temp_grid.walls[idx] {
                 cmd_tx.send(AudioCommand::AddWall { x, y })?;
            }
        }
    }

    // 4. Drive Simulation (if silent)
    // If model is present (fallback), spawn a thread to drive it.
    if let Some(mut model) = audio_system.model.take() {
        std::thread::spawn(move || {
            let mut dummy_buffer = vec![0.0; 1024];
            loop {
                let start = std::time::Instant::now();
                model.process(&mut dummy_buffer);

                // Approximate 44100Hz timing
                // 1024 samples / 44100 Hz = 23.2 ms
                let target_duration = std::time::Duration::from_micros(23220);

                let elapsed = start.elapsed();
                if elapsed < target_duration {
                     std::thread::sleep(target_duration - elapsed);
                }
            }
        });
    }

    // 5. Run TUI
    // We clone lines and entities? App takes ownership.
    let app = App::new(args.width, args.height, cmd_tx, snap_rx, entities, lines);

    // We need to keep audio_system alive if it holds a stream
    // The drop of audio_system would drop the stream.
    // App::run consumes app, but audio_system lives in main stack frame.
    app.run()?;

    Ok(())
}
