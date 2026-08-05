//! # Neuro Strings 🎻🧠
//!
//! **Lineage:** `crates/neuro-sim` (Spiking Neural Network) × `experiments/ferrous-strings` (Acoustic-Magnetic Physics).
//!
//! ## Concept
//! A set of vibrating strings that generate magnetic fields. A Spiking Neural Network (SNN) controls the strings and listens to the environment.
//!
//! The system is a bidirectional bio-acoustic feedback loop:
//! 1. **Brain to Matter (Motor Output):** When specific neurons spike, they physically "pluck" the strings, creating continuous acoustic vibrations and magnetic fields.
//! 2. **Matter to Brain (Sensory Input):** The strings magnetize a continuous fluid `Platter`. The total magnetic flux acts as an input current to the neural network's sensory neurons.
//! 3. **Emergence:** The discrete neural spikes generate continuous magnetic waves, and the magnetic waves dictate the future firing rate of the network. This creates dynamic, self-sustaining rhythmic loops.
//!
//! ## Controls
//! - **Mouse Click:** Stimulate the sensory neuron manually.
//! - **Audio:** Enabled if `cpal` is present (default).
//!
//! ## Emergent Phenotype
//! - **Bio-Acoustic Rhythm:** The network naturally discovers stable rhythmic patterns where the delayed magnetic feedback reinforces the firing sequence.
//! - **Chord Generation:** Multiple neurons can synchronize their firing, plucking different strings simultaneously to produce chords.
//!
use ferrous_core::Platter;
use macroquad::prelude::*;

mod audio;
mod particle;
mod string;

use audio::{init_audio, AudioCommand};
use neuro_sim::Network;
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

    // Initialize strings
    for i in 0..STRING_COUNT {
        let x = 100.0 + i as f32 * STRING_SPACING;
        let pos = vec2(x, 100.0);
        // Start with random frequencies around a scale (target is just a guideline for mutation range)
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(FerrousString::new(pos, 400.0, target_freq));
    }

    // Initialize Particles
    for _ in 0..PARTICLE_COUNT {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        particles.push(Particle::new(x, y));
    }

    let mut network = Network::new();
    let mut string_neurons = Vec::new();
    for i in 0..STRING_COUNT {
        let n = network.add_neuron();
        string_neurons.push(n);
        // Connect neurons in a ring to create a sequence
        if i > 0 {
            network.add_synapse_with_delay(string_neurons[i - 1], n, 20.0, 50); // Excitation
            network.add_synapse_with_delay(n, string_neurons[i - 1], -5.0, 10); // Inhibition backward
        }
    }
    // Close the ring
    if STRING_COUNT > 1 {
        network.add_synapse_with_delay(
            string_neurons[STRING_COUNT - 1],
            string_neurons[0],
            20.0,
            50,
        );
    }

    // Add sensory neurons that feed from the platter
    let sensory_n = network.add_neuron();
    for n in &string_neurons {
        network.add_synapse(sensory_n, *n, 5.0);
    }

    let mut prev_mouse = vec2(0.0, 0.0);

    // Texture for Platter Heatmap
    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();

        let (mx, my) = mouse_position();
        let mouse_pos = vec2(mx, my);
        let mouse_delta = mouse_pos - prev_mouse;
        let mouse_speed = mouse_delta.length();

        // --- Physics Update ---

        // 1. Decay Platter
        platter.decay(0.99); // Slow decay to let magnetism build up

        // 2. Update Strings (Magnetize Platter)
        for s in &mut strings {
            // Apply magnetic forces from platter to string
            s.update_physics(dt, &platter, grid_scale);

            // Apply string vibration to platter
            // This is the bidirectional coupling!
            // String writes to Platter
            // Platter reads from String (in update_physics)
            // But we do it in steps.

            // To make it interesting, let's say the string ONLY writes to platter if vibrating
            if s.vibration.abs() > 0.1 {
                // Determine grid cells under the string
                // For simplicity, just a few points along the string
                let steps = 20;
                let step_size = s.length / steps as f32;

                for i in 0..=steps {
                    let y_offset = i as f32 * step_size;
                    let ratio = y_offset / s.length;
                    let shape = (std::f32::consts::PI * ratio).sin();
                    let x = s.pos.x + s.vibration * shape;
                    let y = s.pos.y + y_offset;

                    let gx = (x / grid_scale) as i32;
                    let gy = (y / grid_scale) as i32;

                    if gx >= 0 && gy >= 0 && gx < grid_w as i32 && gy < grid_h as i32 {
                        // Polarity flips based on vibration direction
                        let val = (s.vibration * 0.01 * shape) as f64;
                        platter.magnetize(gx as usize, gy as usize, val);
                    }
                }
            }

            // Mouse Interaction (Plucking)
            let string_x = s.pos.x + s.vibration; // Approximate
                                                  // Check cross
            let crossed = (prev_mouse.x < string_x && mouse_pos.x >= string_x)
                || (prev_mouse.x > string_x && mouse_pos.x <= string_x);
            let in_range = mouse_pos.y >= s.pos.y && mouse_pos.y <= s.pos.y + s.length;

            if crossed && in_range {
                let strength = mouse_speed.clamp(5.0, 50.0);
                let direction = if mouse_delta.x > 0.0 { 1.0 } else { -1.0 };
                s.pluck(strength * direction);

                // Play Audio
                let _ = cmd_tx.send(AudioCommand {
                    frequency: s.frequency,
                    decay: s.decay,
                    amplitude: (strength / 50.0).clamp(0.1, 0.8),
                });
            }
        }

        // --- Neural Network Update ---
        // Calculate total magnetic flux
        let mut total_flux = 0.0;
        for y in 0..grid_h {
            for x in 0..grid_w {
                total_flux += platter.get_magnetism(x, y).abs();
            }
        }

        let mut inputs = vec![0.0; network.neurons.len()];
        // Inject baseline current to keep it going, plus sensory input from magnetic flux
        inputs[sensory_n] = 5.0 + (total_flux as f32 * 0.01).clamp(0.0, 20.0);

        // Let user mouse click stimulate sensory neuron
        if is_mouse_button_down(MouseButton::Left) {
            inputs[sensory_n] += 50.0;
        }

        // We run multiple physics steps per frame for smooth audio and network speed
        let sub_steps = 10;

        for _ in 0..sub_steps {
            network.step(&inputs);

            for (i, &n_idx) in string_neurons.iter().enumerate() {
                if network.is_spiking(n_idx) {
                    strings[i].pluck(40.0);
                    // Play Audio
                    let _ = cmd_tx.send(AudioCommand {
                        frequency: strings[i].frequency,
                        decay: strings[i].decay,
                        amplitude: 0.8,
                    });
                }
            }
        }

        // 3. Update Particles
        for p in &mut particles {
            p.update(&platter, grid_scale, dt);
        }

        // --- Evolution ---

        // --- Render ---
        clear_background(BLACK);

        // 1. Draw Platter (Heatmap)
        let mut image = texture.get_texture_data();
        for y in 0..grid_h {
            for x in 0..grid_w {
                let mag = platter.get_magnetism(x, y); // 0.0 to 1.0

                let val = mag as f32;
                let c = if val < 0.0 {
                    // Blue for negative
                    Color::new(0.0, 0.0, (-val).clamp(0.0, 1.0), 1.0)
                } else {
                    // Red for positive
                    Color::new(val.clamp(0.0, 1.0), 0.0, 0.0, 1.0)
                };

                image.set_pixel(x as u32, y as u32, c);
            }
        }
        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(grid_w as f32 * grid_scale, grid_h as f32 * grid_scale)),
                ..Default::default()
            },
        );

        // 2. Draw Particles
        for p in &particles {
            p.draw();
        }

        // 3. Draw Strings
        for s in &strings {
            s.draw();
        }

        // UI
        draw_text("Neuro Strings", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Strings vibrate -> Magnetic Field -> Particles Flow",
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Neural Spikes -> Acoustic Plucks -> Magnetic Particles",
            10.0,
            70.0,
            20.0,
            GRAY,
        );
        draw_text("Pluck with Mouse!", 10.0, 90.0, 20.0, YELLOW);

        prev_mouse = mouse_pos;
        next_frame().await;
    }
}
