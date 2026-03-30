//! # 🧬 Spiking Acoustic Symbiosis (neuro-strings)
//!
//! **Lineage:**
//! - Parent A (Biological): `crates/neuro-sim` (Izhikevich Spiking Neural Network). This provides the discrete, biologically realistic neural firing patterns, synapse dynamics, and action potentials.
//! - Parent B (Physical): `experiments/ferrous-strings` (Acoustic-Magnetic Strings). This provides the continuous acoustic string physics, wave propagation, and the macroscopic magnetic fluid medium (`Platter`).
//!
//! **Phenotype:**
//! A bio-acoustic instrument where neural assemblies synchronize to create rhythmic magnetic pulses. By bridging the discrete neural spikes with continuous string physics, we establish an emergent behavior where neural firing translates directly into physical string resonance, which in turn physically pushes and pulls magnetic particles across the visual space.

use ferrous_core::Platter;
use macroquad::prelude::*;
use neuro_sim::Network;

mod audio;
mod particle;
mod string;

use audio::{init_audio, AudioCommand};
use particle::Particle;
use string::FerrousString;

const STRING_COUNT: usize = 8;
const PARTICLE_COUNT: usize = 500;
const STRING_SPACING: f32 = 100.0;
const BASE_FREQ: f32 = 110.0; // A2

#[macroquad::main("Neuro Strings")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    // Keep handle alive
    let _audio_handle = audio_handle;

    let grid_w = 200;
    let grid_h = 150;
    let mut platter = Platter::new(grid_w, grid_h);
    let grid_scale = 4.0;

    let mut strings: Vec<FerrousString> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut net = Network::new();
    let mut string_neurons = Vec::new();

    let start_x = 100.0;
    let y_pos = 100.0;

    for i in 0..STRING_COUNT {
        let x = start_x + (i as f32) * STRING_SPACING;
        let s = FerrousString::new(vec2(x, y_pos), 400.0, BASE_FREQ * (i + 1) as f32 * 0.5);

        let n = net.add_neuron();
        string_neurons.push(n);

        strings.push(s);
    }

    // Connect some neurons together for rhythmic firing
    for i in 0..STRING_COUNT {
        for j in 0..STRING_COUNT {
            if i != j && ::macroquad::rand::gen_range(0.0, 1.0) < 0.3 {
                // Random synapses with delay
                let weight = ::macroquad::rand::gen_range(-5.0, 10.0);
                let delay = ::macroquad::rand::gen_range(1, 10);
                net.add_synapse_with_delay(string_neurons[i], string_neurons[j], weight, delay);
            }
        }
    }

    for _ in 0..PARTICLE_COUNT {
        let x = ::macroquad::rand::gen_range(0.0, 800.0);
        let y = ::macroquad::rand::gen_range(0.0, 600.0);
        particles.push(Particle::new(x, y));
    }

    let mut inputs = vec![0.0; STRING_COUNT];

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        // 1. Spiking Neural Network Step
        // Reset inputs
        inputs.fill(0.0);

        // Randomly inject current to stimulate network
        if ::macroquad::rand::gen_range(0.0, 1.0) < 0.1 {
            let idx = ::macroquad::rand::gen_range(0, STRING_COUNT as u32) as usize;
            inputs[idx] = 20.0; // Inject strong current
        }

        net.step(&inputs);

        // Check spiking and pluck strings
        for i in 0..STRING_COUNT {
            if net.is_spiking(string_neurons[i]) {
                strings[i].pluck(100.0);

                // Play audio
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    frequency: strings[i].frequency,
                    amplitude: 0.5,
                    decay: strings[i].decay,
                });
            }
        }

        // 2. Physics & Magnetism
        platter.decay(0.95);

        for s in strings.iter_mut() {
            s.update_physics(dt, &platter, grid_scale);
            s.magnetize_platter(&mut platter, grid_scale);

            if s.vibration.abs() > 5.0 {
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    frequency: s.frequency,
                    amplitude: s.magnetic_strength * 0.1,
                    decay: s.decay,
                });
            }
        }

        // Update particles
        for p in &mut particles {
            p.update(&platter, grid_scale, dt);
        }

        // 3. Draw
        // Draw platter field
        for y in 0..grid_h {
            for x in 0..grid_w {
                let mag = platter.get_magnetism(x, y);
                if mag > 0.05 {
                    let screen_x = x as f32 * grid_scale;
                    let screen_y = y as f32 * grid_scale;
                    let c = mag as f32;
                    draw_rectangle(
                        screen_x,
                        screen_y,
                        grid_scale,
                        grid_scale,
                        Color::new(0.2, c * 0.8, c, 0.3),
                    );
                }
            }
        }

        // Draw strings
        for s in &strings {
            s.draw();
        }

        // Draw particles
        for p in &particles {
            p.draw();
        }

        // Draw HUD
        draw_text("NEURO-STRINGS", 10.0, 30.0, 30.0, WHITE);
        draw_text("Spiking Neural Strings", 10.0, 50.0, 20.0, GRAY);
        draw_text(
            "Neurons pluck strings. Strings emit magnetism.",
            10.0,
            70.0,
            20.0,
            GRAY,
        );
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 90.0, 20.0, GRAY);

        next_frame().await;
    }
}
