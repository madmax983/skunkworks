use macroquad::prelude::*;
use crate::network::{Network, NeuronType};
use crate::audio::AudioEngine;

mod network;
mod audio;

#[macroquad::main("Polyrhythmic Cortex")]
async fn main() {
    // Initialize random seed
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut network = Network::new(200);
    // Initialize audio (might fail on some systems, but we proceed)
    let audio = AudioEngine::new();

    // Select Output neurons (first 8 excitatory neurons found)
    let mut output_neurons = Vec::new();
    let mut count = 0;
    for (i, t) in network.neuron_types.iter().enumerate() {
        if *t == NeuronType::Excitatory {
            output_neurons.push(i);
            count += 1;
            if count >= 8 { break; }
        }
    }

    // Pentatonic scale frequencies (C Major Pentatonic)
    // C4, D4, E4, G4, A4, C5, D5, E5
    let scale = [261.63, 293.66, 329.63, 392.00, 440.00, 523.25, 587.33, 659.25];

    loop {
        // Handle Input
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Map mouse Y to neuron index range
            // Inject into a small cluster around the cursor
            let ratio = my / screen_height();
            let center_idx = (ratio * network.neurons.len() as f32) as usize;
            let radius = 5;

            for i in (center_idx.saturating_sub(radius))..=(center_idx + radius).min(network.neurons.len() - 1) {
                 network.inject(i, 50.0);
            }
        }

        if is_key_pressed(KeyCode::R) {
            network = Network::new(200);
            output_neurons.clear();
            count = 0;
            for (i, t) in network.neuron_types.iter().enumerate() {
                if *t == NeuronType::Excitatory {
                    output_neurons.push(i);
                    count += 1;
                    if count >= 8 { break; }
                }
            }
        }

        // Update Network
        let steps_per_frame = 5; // Speed up simulation

        for _ in 0..steps_per_frame {
            network.update();

            // Check output neurons
            for (i, &neuron_idx) in output_neurons.iter().enumerate() {
                 let last_step = network.current_step.saturating_sub(1);
                 // Check if it spiked in the last step
                 if let Some(t) = network.last_spike_times[neuron_idx] {
                     if t == last_step {
                         // Play tone
                         // Cycle through scale if we have more neurons than notes, or use modulo
                         let freq = scale[i % scale.len()];
                         audio.play_tone(freq, 100);
                     }
                 }
            }
        }

        // Draw
        clear_background(BLACK);

        // Draw Raster Plot
        // X: Time (scrolling window), Y: Neuron Index
        let window_size = 600;
        let start_time = network.current_step.saturating_sub(window_size);

        // Optimization: Find start index in spike_history roughly
        // spike_history is ordered by time.
        // We can just iterate reverse until we hit start_time.

        for &(t, idx) in network.spike_history.iter().rev() {
            if t < start_time {
                break;
            }
            let time_offset = t - start_time;
            let x = time_offset as f32 / window_size as f32 * screen_width();
            let y = (idx as f32 / network.neurons.len() as f32) * (screen_height() - 100.0) + 10.0; // Reserve bottom for UI

            let color = match network.neuron_types[idx] {
                NeuronType::Excitatory => GREEN,
                NeuronType::Inhibitory => RED,
            };
            draw_circle(x, y, 2.0, color);
        }

        // Draw Output Indicators (Piano keys / Drum pads)
        let indicator_y = screen_height() - 50.0;
        let spacing = screen_width() / output_neurons.len() as f32;

        for (i, &idx) in output_neurons.iter().enumerate() {
             let x = i as f32 * spacing + spacing / 2.0;
             // Check if active recently (in last 10 steps)
             let active = if let Some(t) = network.last_spike_times[idx] {
                 network.current_step.saturating_sub(t) < 10
             } else { false };

             let color = if active { YELLOW } else { GRAY };
             draw_circle(x, indicator_y, 20.0, color);
             draw_text(&format!("{}", i + 1), x - 5.0, indicator_y + 5.0, 20.0, BLACK);

             // Draw frequency
             draw_text(&format!("{:.0}Hz", scale[i % scale.len()]), x - 15.0, indicator_y + 35.0, 15.0, WHITE);
        }

        draw_text("Polyrhythmic Cortex", 10.0, 20.0, 30.0, WHITE);
        draw_text("Left Click: Stimulate | R: Reset", 10.0, 40.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Step: {}", network.current_step), screen_width() - 150.0, 20.0, 20.0, WHITE);

        next_frame().await;
    }
}
