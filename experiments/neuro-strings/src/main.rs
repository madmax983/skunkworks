use ferrous_core::Platter;
use macroquad::prelude::*;
use neuro_sim::Network;

mod audio;
mod string;

use audio::{init_audio, AudioCommand};
use string::FerrousString;

const STRING_COUNT: usize = 8;
const BASE_FREQ: f32 = 110.0; // A2

#[macroquad::main("Neuro Strings")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    let _audio_handle = audio_handle;

    let grid_w = 200;
    let grid_h = 150;
    let mut platter = Platter::new(grid_w, grid_h);
    let grid_scale = 4.0;

    let mut strings: Vec<FerrousString> = Vec::new();
    let string_spacing = (grid_w as f32 * grid_scale) / (STRING_COUNT as f32 + 1.0);

    for i in 0..STRING_COUNT {
        let x = (i as f32 + 1.0) * string_spacing;
        let pos = vec2(x, 100.0);
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(FerrousString::new(pos, 400.0, target_freq));
    }

    // Initialize Neural Network
    let mut brain = Network::new();
    let num_neurons = STRING_COUNT;

    for _ in 0..num_neurons {
        brain.add_neuron();
    }

    // Connect neurons to create a rhythmic circuit
    for i in 0..num_neurons {
        let next = (i + 1) % num_neurons;
        // Excitatory loop
        brain.add_synapse_with_delay(i, next, 25.0, 5);
        // Inhibitory feedback
        let prev = (i + num_neurons - 1) % num_neurons;
        brain.add_synapse_with_delay(i, prev, -10.0, 2);
    }

    // Kickstart the first neuron
    let mut input_currents = vec![0.0; num_neurons];
    input_currents[0] = 50.0;

    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();

        // 1. Update Spiking Neural Network
        brain.step(&input_currents);

        // Reset external inputs after kickstart
        input_currents.fill(0.0);

        // 2. Map Neural Spikes to String Plucks
        for (i, s) in strings.iter_mut().enumerate() {
            if brain.is_spiking(i) {
                let strength = 30.0; // Pluck strength
                s.pluck(strength);

                let _ = cmd_tx.send(AudioCommand::Pluck {
                    frequency: s.frequency,
                    decay: s.decay,
                    amplitude: 0.8,
                });
            }
        }

        // 3. Update Platter Physics
        platter.decay(0.99);

        for s in strings.iter_mut() {
            s.update_physics(dt, &platter, grid_scale);

            if s.vibration.abs() > 0.1 {
                let steps = 20;
                let step_size = s.length / steps as f32;

                for j in 0..=steps {
                    let y_offset = j as f32 * step_size;
                    let ratio = y_offset / s.length;
                    let shape = (std::f32::consts::PI * ratio).sin();
                    let x = s.pos.x + s.vibration * shape;
                    let y = s.pos.y + y_offset;

                    let gx = (x / grid_scale) as i32;
                    let gy = (y / grid_scale) as i32;

                    if gx >= 0 && gy >= 0 && gx < grid_w as i32 && gy < grid_h as i32 {
                        let val = (s.vibration * 0.01 * shape) as f64;
                        platter.magnetize(gx as usize, gy as usize, val);
                    }
                }
            }
        }

        // 4. Map Platter Magnetic Field back to Synaptic Weights (Plasticity)
        // Strings create magnetic fields on the platter.
        // If a synapse connects neuron A -> B, check the magnetic field between string A and string B.
        for syn in brain.synapses.iter_mut() {
            let s_from = &strings[syn.from];
            let s_to = &strings[syn.to];

            // Sample midpoint on the platter
            let mid_x = (s_from.pos.x + s_to.pos.x) / 2.0;
            let gx = (mid_x / grid_scale) as i32;
            let gy = (150.0 / grid_scale) as i32; // Arbitrary sample height

            if gx >= 0 && gy >= 0 && gx < grid_w as i32 && gy < grid_h as i32 {
                let mag = platter.get_magnetism(gx as usize, gy as usize) as f32;

                // Modulate weight based on magnetism (Hebbian-like acoustic learning)
                syn.weight += mag * 0.1 * dt;

                // Keep weights bounded
                syn.weight = syn.weight.clamp(-50.0, 50.0);
            }
        }

        // --- Render ---
        clear_background(BLACK);

        let mut image = texture.get_texture_data();
        for y in 0..grid_h {
            for x in 0..grid_w {
                let mag = platter.get_magnetism(x, y);

                let val = mag as f32;
                let c = if val < 0.0 {
                    Color::new(0.0, 0.0, (-val).clamp(0.0, 1.0), 1.0)
                } else {
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

        for s in &strings {
            s.draw();
        }

        draw_text("Neuro Strings", 10.0, 30.0, 30.0, WHITE);
        draw_text("Spiking Neural Network -> Physical String Plucks", 10.0, 50.0, 20.0, GRAY);
        draw_text("String Vibration -> Magnetic Field -> Synaptic Plasticity", 10.0, 70.0, 20.0, GRAY);

        next_frame().await;
    }
}
