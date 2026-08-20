use anyhow::Result;
use crossbeam_channel::unbounded;
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use resonance_audio::{AudioCommand, AudioModel};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing TUI.");
        return Ok(());
    }

    let width = 50;
    let height = 50;

    let (command_tx, command_rx) = unbounded();
    let (snapshot_tx, _snapshot_rx) = unbounded();
    let mut audio_model = AudioModel::new(width, height, command_rx, snapshot_tx, None);

    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    let num_boids = 10;

    for _ in 0..num_boids {
        positions.push(Vec2::new(
            rand::random::<f64>() * width as f64,
            rand::random::<f64>() * height as f64,
        ));
        velocities.push(Vec2::new(
            (rand::random::<f64>() - 0.5) * 2.0,
            (rand::random::<f64>() - 0.5) * 2.0,
        ));
    }

    let params = FlockingParams {
        view_radius: 5.0,
        separation_radius: 2.0,
        max_speed: 1.0,
        max_force: 0.1,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    println!("Starting simulation... Press Ctrl+C to exit.");
    for _ in 0..100 {
        let mut new_velocities = velocities.clone();
        for i in 0..num_boids {
            let force = compute_force(&positions, &velocities, i, &params);
            new_velocities[i].x += force.x;
            new_velocities[i].y += force.y;

            let speed = (new_velocities[i].x * new_velocities[i].x
                + new_velocities[i].y * new_velocities[i].y)
                .sqrt();
            if speed > params.max_speed {
                new_velocities[i].x = (new_velocities[i].x / speed) * params.max_speed;
                new_velocities[i].y = (new_velocities[i].y / speed) * params.max_speed;
            }

            positions[i].x += new_velocities[i].x;
            positions[i].y += new_velocities[i].y;

            let px = positions[i].x.clamp(0.0, (width - 1) as f64) as usize;
            let py = positions[i].y.clamp(0.0, (height - 1) as f64) as usize;

            let frequency = 200.0 + (i as f32 * 50.0);

            let _ = command_tx.send(AudioCommand::Oscillate {
                x: px,
                y: py,
                frequency,
                strength: 0.5,
            });
        }
        velocities = new_velocities;

        // Process a block of audio (simulating time passing)
        let mut output = vec![0.0; 256];
        audio_model.process(&mut output);
    }

    Ok(())
}
