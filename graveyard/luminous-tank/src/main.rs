//! # Acoustic Swarming (luminous-tank)
//!
//! **Lineage:** `luminous-flock` × `ripple-tank`
//!
//! A hybrid created by The Splice Surgeon.
//! The boids from `luminous-flock` are placed within the 2D acoustic wave simulation
//! from `ripple-tank`. As the flock navigates the environment, they disturb the acoustic
//! medium, plucking the wave tank. In turn, the gradient of the acoustic pressure waves
//! physically pushes the boids.
//!
//! **Emergent Phenotype:**
//! The bidirectional feedback between the swarming intelligence and the physical acoustic
//! field results in "Acoustic Flocking". The flock generates standing waves in the tank,
//! and the standing waves organize the flock into cymatic patterns.

use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use resonance_audio::audio::AudioSnapshot;

use locus::flocking::{compute_force, FlockingParams};
use locus::Vec2;
use std::f64::consts::TAU;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;
const NUM_BOIDS: usize = 400;

struct Boid {
    position: Vec2,
    velocity: Vec2,
}

#[macroquad::main("Luminous Tank")]
async fn main() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    #[cfg(feature = "audio")]
    let mut _stream_opt = None;
    #[cfg(feature = "audio")]
    let mut _fallback_model = None;

    #[cfg(feature = "audio")]
    match init_audio(cmd_rx.clone(), snap_tx.clone()) {
        Ok(s) => _stream_opt = Some(s),
        Err(e) => {
            eprintln!("Audio init failed: {}. Running in silent mode.", e);
            _fallback_model = Some(AudioModel::new(
                GRID_WIDTH,
                GRID_HEIGHT,
                cmd_rx,
                snap_tx,
                None,
            ));
        }
    }

    #[cfg(not(feature = "audio"))]
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

    let mut boids: Vec<Boid> = (0..NUM_BOIDS)
        .map(|_| {
            let angle = ::rand::random::<f64>() * TAU;
            Boid {
                position: Vec2::new(
                    ::rand::random::<f64>() * GRID_WIDTH as f64,
                    ::rand::random::<f64>() * GRID_HEIGHT as f64,
                ),
                velocity: Vec2::new(angle.cos() * 2.0, angle.sin() * 2.0),
            }
        })
        .collect();

    let flocking_params = FlockingParams {
        view_radius: 10.0,
        separation_radius: 4.0,
        max_speed: 2.0,
        max_force: 0.1,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        #[cfg(not(feature = "audio"))]
        {
            let dt = get_frame_time();
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000);
            let mut dummy_buffer = vec![0.0; steps];
            model.process(&mut dummy_buffer);
        }

        #[cfg(feature = "audio")]
        if let Some(ref mut fallback) = _fallback_model {
            let dt = get_frame_time();
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000);
            let mut dummy_buffer = vec![0.0; steps];
            fallback.process(&mut dummy_buffer);
        }

        let positions: Vec<Vec2> = boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = boids.iter().map(|b| b.velocity).collect();

        // Update boids
        for (i, boid) in boids.iter_mut().enumerate().take(NUM_BOIDS) {
            let mut force = compute_force(&positions, &velocities, i, &flocking_params);

            // Advection from acoustic waves (gradient)
            let bx = boid.position.x as i32;
            let by = boid.position.y as i32;

            if bx > 0 && bx < GRID_WIDTH as i32 - 1 && by > 0 && by < GRID_HEIGHT as i32 - 1 {
                let idx = (by as usize) * GRID_WIDTH + (bx as usize);
                let idx_dx = idx + 1;
                let idx_dy = idx + GRID_WIDTH;

                let p = current_snapshot[idx];
                let p_dx = current_snapshot[idx_dx];
                let p_dy = current_snapshot[idx_dy];

                let grad_x = (p_dx - p) as f64;
                let grad_y = (p_dy - p) as f64;

                // Boids are repelled by high pressure (acoustic advection)
                force.x -= grad_x * 10.0;
                force.y -= grad_y * 10.0;

                // Boids create ripples based on their speed and density
                if ::rand::random::<f64>() < 0.05 {
                    let _ = cmd_tx.send(AudioCommand::Pluck {
                        x: bx as usize,
                        y: by as usize,
                        strength: 0.1,
                    });
                }
            }

            boid.velocity += force;

            // Limit speed
            let speed_sq = boid.velocity.x * boid.velocity.x + boid.velocity.y * boid.velocity.y;
            if speed_sq > flocking_params.max_speed * flocking_params.max_speed {
                let speed = speed_sq.sqrt();
                boid.velocity = (boid.velocity / speed) * flocking_params.max_speed;
            }

            boid.position += boid.velocity;

            // Wrap around edges
            boid.position.x = boid.position.x.rem_euclid(GRID_WIDTH as f64);
            boid.position.y = boid.position.y.rem_euclid(GRID_HEIGHT as f64);
        }

        // Draw wave tank
        for (i, &val) in current_snapshot.iter().enumerate() {
            let x = (i % GRID_WIDTH) as u32;
            let y = (i / GRID_WIDTH) as u32;

            let color = if val > 0.0 {
                Color::new(val.min(1.0), 0.0, 0.0, 1.0)
            } else {
                Color::new(0.0, 0.0, (-val).min(1.0), 1.0)
            };
            image.set_pixel(x, y, color);
        }
        texture.update(&image);

        clear_background(BLACK);

        let scale = (screen_width() / GRID_WIDTH as f32).min(screen_height() / GRID_HEIGHT as f32);
        let draw_width = GRID_WIDTH as f32 * scale;
        let draw_height = GRID_HEIGHT as f32 * scale;
        let offset_x = (screen_width() - draw_width) / 2.0;
        let offset_y = (screen_height() - draw_height) / 2.0;

        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_width, draw_height)),
                ..Default::default()
            },
        );

        // Draw Boids
        for boid in &boids {
            let x = offset_x + boid.position.x as f32 * scale;
            let y = offset_y + boid.position.y as f32 * scale;
            draw_circle(x, y, scale * 0.5, GREEN);
        }

        draw_text("Acoustic Swarming (luminous-tank)", 10.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}

#[cfg(feature = "audio")]
fn init_audio(
    cmd_rx: crossbeam_channel::Receiver<AudioCommand>,
    snap_tx: crossbeam_channel::Sender<AudioSnapshot>,
) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

    let config = device.default_output_config()?;

    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow::anyhow!("Only F32 sample format supported")),
    };

    stream.play()?;
    Ok(stream)
}
