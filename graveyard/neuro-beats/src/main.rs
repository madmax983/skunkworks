use macroquad::prelude::*;
use neuro_sim::Network;
// use neuro_sim::Izhikevich; // Not directly needed if we access via Network

mod audio;
use audio::AudioEngine;

struct NeuronData {
    pos: Vec2,
    frequency: f32,
    last_spike_time: Option<f32>,
}

#[macroquad::main("Neuro Beats")]
async fn main() {
    let audio_engine = AudioEngine::new();
    let mut network = Network::new();
    let mut neuron_data: Vec<NeuronData> = Vec::new();

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

        // Add neuron to network
        network.add_neuron();

        // Add metadata
        neuron_data.push(NeuronData {
            pos,
            frequency: freq,
            last_spike_time: None,
        });
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
        network.add_synapse_with_delay(i, next, 50.0, 10); // 10 ticks delay
    }

    // Connect cross connections for complexity
    for i in 0..num_neurons {
        let other = (i + num_neurons / 2) % num_neurons;
        network.add_synapse_with_delay(i, other, 30.0, 20);
    }

    loop {
        clear_background(BLACK);

        // let dt = 1.0; // Simulation time step (implied by neuro-sim step)

        // Input Handling
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let mouse_pos = vec2(mx, my);
            // Find closest neuron
            let mut closest = None;
            let mut min_dist = f32::MAX;

            for (i, data) in neuron_data.iter().enumerate() {
                let d = data.pos.distance(mouse_pos);
                if d < min_dist {
                    min_dist = d;
                    closest = Some(i);
                }
            }

            if let Some(id) = closest {
                if min_dist < 50.0 {
                    // Inject current
                    network.neurons[id].inject(100.0);
                    // Also play a sound immediately for feedback
                    audio_engine.play_tone(neuron_data[id].frequency, 0.1);
                }
            }
        }

        // Space to reset/inject all
        if is_key_pressed(KeyCode::Space) {
            for neuron in &mut network.neurons {
                neuron.inject(50.0);
            }
        }

        // Update Network
        network.step(&[]);

        // Check for spikes and play audio
        for (i, spiked) in network.spikes.iter().enumerate() {
            if *spiked {
                let data = &mut neuron_data[i];
                data.last_spike_time = Some(get_time() as f32);
                // Play short blip
                audio_engine.play_tone(data.frequency, 0.1);
            }
        }

        // Draw Synapses
        for (syn_idx, synapse) in network.synapses.iter().enumerate() {
            let start = neuron_data[synapse.from].pos;
            let end = neuron_data[synapse.to].pos;

            let is_active = network.get_synapse_activity(syn_idx);

            let color = if is_active {
                YELLOW
            } else {
                // Visualize signal traveling?
                // neuro-sim doesn't expose `timer` directly easily (it's inside spikes_in_transit).
                // But we can check if `spikes_in_transit` is not empty.
                if !synapse.spikes_in_transit.is_empty() {
                    // Just draw a blip somewhere?
                    // Without exact timer, we can't interpolate perfectly.
                    // But we can just draw the line brighter.
                    Color::new(0.5, 0.5, 0.5, 1.0)
                } else {
                    DARKGRAY
                }
            };

            draw_line(start.x, start.y, end.x, end.y, 1.0, color);

            // If active, draw a circle at destination
            if is_active {
                draw_circle(end.x, end.y, 5.0, YELLOW);
            }
        }

        // Draw Neurons
        for (i, neuron) in network.neurons.iter().enumerate() {
            let data = &neuron_data[i];

            // Map voltage to color
            // v is typically -65 to 30.
            let v = neuron.v;
            let normalized = ((v + 80.0) / 110.0).clamp(0.0, 1.0);

            let color = Color::new(normalized, 0.2, 1.0 - normalized, 1.0);

            // Flash white on spike
            let draw_color = if let Some(last) = data.last_spike_time {
                if get_time() as f32 - last < 0.1 {
                    WHITE
                } else {
                    color
                }
            } else {
                color
            };

            draw_circle(data.pos.x, data.pos.y, 10.0, draw_color);
            draw_circle_lines(data.pos.x, data.pos.y, 10.0, 2.0, LIGHTGRAY);
        }

        draw_text(
            "Click neuron to stimulate. SPACE to stimulate all.",
            20.0,
            30.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}
