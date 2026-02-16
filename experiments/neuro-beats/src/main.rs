use macroquad::prelude::*;

mod audio;
mod neuron;

use audio::AudioEngine;
use neuron::Network;

#[macroquad::main("Neuro Beats")]
async fn main() {
    let audio_engine = AudioEngine::new();
    let mut network = Network::new();

    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let radius = 200.0;
    let num_neurons = 12;

    // Pentatonic scale frequencies (approximate)
    let base_freq = 220.0;
    let scale_ratios = [1.0, 1.125, 1.25, 1.5, 1.667];

    for i in 0..num_neurons {
        let angle = i as f32 * 2.0 * std::f32::consts::PI / num_neurons as f32;
        let pos = center + vec2(angle.cos(), angle.sin()) * radius;

        // Map index to scale
        let octave = (i / 5) as f32;
        let ratio = scale_ratios[i % 5];
        let freq = base_freq * ratio * 2.0f32.powf(octave);

        network.add_neuron(pos, freq);
    }

    // Connect in a ring
    for i in 0..num_neurons {
        let next = (i + 1) % num_neurons;
        // Weight needs to be high enough to trigger a spike.
        // Resting -65, Threshold 30. Delta = 95.
        // Izhikevich parameters a=0.02, b=0.2.
        // Input adds to v' = ... + I.
        // With substeps=2, dt=1.0 roughly.
        // Let's try 50.0.
        network.add_synapse(i, next, 50.0, 10); // 10 ticks delay
    }

    // Connect cross connections for complexity
    for i in 0..num_neurons {
         let other = (i + num_neurons / 2) % num_neurons;
         network.add_synapse(i, other, 30.0, 20);
    }

    loop {
        clear_background(BLACK);

        let dt = 1.0; // Simulation time step (not necessarily wall clock)

        // Input Handling
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let mouse_pos = vec2(mx, my);
            // Find closest neuron
            let mut closest = None;
            let mut min_dist = f32::MAX;

            for neuron in &network.neurons {
                let d = neuron.pos.distance(mouse_pos);
                if d < min_dist {
                    min_dist = d;
                    closest = Some(neuron.id);
                }
            }

            if let Some(id) = closest {
                if min_dist < 50.0 {
                    // Inject current
                    network.neurons[id].physics.inject(100.0);
                    // Also play a sound immediately for feedback
                    audio_engine.play_tone(network.neurons[id].frequency, 0.1);
                }
            }
        }

        // Space to reset/inject all
        if is_key_pressed(KeyCode::Space) {
             for neuron in &mut network.neurons {
                 neuron.physics.inject(50.0);
             }
        }

        // Update Network
        let spikes = network.update(dt, get_time() as f32);

        // Audio
        for &id in &spikes {
            let neuron = &network.neurons[id];
            // Play short blip
            audio_engine.play_tone(neuron.frequency, 0.1);
        }

        // Draw Synapses
        for synapse in &network.synapses {
            let start = network.neurons[synapse.from].pos;
            let end = network.neurons[synapse.to].pos;

            let color = if synapse.active {
                YELLOW
            } else if synapse.timer > 0 {
                // Signal traveling
                let t = synapse.timer as f32 / synapse.delay as f32;
                // Interpolate position
                let pos = start + (end - start) * (1.0 - t);
                draw_circle(pos.x, pos.y, 3.0, WHITE);
                GRAY
            } else {
                DARKGRAY
            };

            draw_line(start.x, start.y, end.x, end.y, 1.0, color);
        }

        // Draw Neurons
        for neuron in &network.neurons {
            // Map voltage to color
            // v is typically -65 to 30.
            let v = neuron.physics.v;
            let normalized = ((v + 80.0) / 110.0).clamp(0.0, 1.0);

            let color = Color::new(normalized, 0.2, 1.0 - normalized, 1.0);

            // Flash white on spike
            let draw_color = if let Some(last) = neuron.last_spike_time {
                if get_time() as f32 - last < 0.1 {
                    WHITE
                } else {
                    color
                }
            } else {
                color
            };

            draw_circle(neuron.pos.x, neuron.pos.y, 10.0, draw_color);
            draw_circle_lines(neuron.pos.x, neuron.pos.y, 10.0, 2.0, LIGHTGRAY);
        }

        draw_text("Click neuron to stimulate. SPACE to stimulate all.", 20.0, 30.0, 20.0, WHITE);

        next_frame().await;
    }
}
