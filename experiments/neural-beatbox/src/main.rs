use macroquad::prelude::*;
use ::rand::Rng;

mod brain;
mod synth;

use brain::{Brain, NeuronType};
use synth::{init_audio, AudioCommand, Waveform};

const NEURON_RADIUS: f32 = 4.0;
const GRID_COLS: usize = 30;
const GRID_ROWS: usize = 20;
const SPACING: f32 = 25.0;

fn get_pentatonic_freq(index: usize) -> f32 {
    let base = 110.0; // A2
    let pentatonic_ratios = [1.0, 1.2, 1.333, 1.5, 1.714, 2.0]; // Approx 0, 3, 5, 7, 10 semitones
    let octave = index / 5;
    let note = index % 5;
    base * (2.0f32).powi(octave as i32) * pentatonic_ratios[note]
}

#[macroquad::main("Neural Beatbox")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    // Keep handle alive
    let _audio_handle = audio_handle;

    let mut brain = Brain::new();
    let mut rng = ::rand::thread_rng();

    // 1. Initialize Neurons
    // Grid layout
    let start_x = (screen_width() - (GRID_COLS as f32 * SPACING)) / 2.0;
    let start_y = (screen_height() - (GRID_ROWS as f32 * SPACING)) / 2.0;

    for y in 0..GRID_ROWS {
        for x in 0..GRID_COLS {
            let pos = vec2(start_x + x as f32 * SPACING, start_y + y as f32 * SPACING);

            // 80% Excitatory, 20% Inhibitory
            // Some pacemakers in the center
            let n_type = if rng.gen::<f32>() < 0.05 {
                NeuronType::Pacemaker
            } else if rng.gen::<f32>() < 0.2 {
                NeuronType::Inhibitory
            } else {
                NeuronType::Excitatory
            };

            brain.add_neuron(pos, n_type);
        }
    }

    // 2. Connect Neurons (Small World / Random)
    // Connect each neuron to K neighbors and some long-range
    for i in 0..brain.neurons.len() {
        // Local connections
        let my_pos = brain.neurons[i].pos;
        for j in 0..brain.neurons.len() {
            if i == j { continue; }
            let dist = my_pos.distance(brain.neurons[j].pos);

            // Probability of connection falls off with distance
            // P = C * exp(-dist/lambda)
            // Actually simpler: connect to nearest neighbors

            if dist < SPACING * 1.5 {
                // Neighbors
                if rng.gen::<f32>() < 0.3 {
                     let weight = match brain.neurons[i].neuron_type {
                        NeuronType::Excitatory => 15.0, // Strong excitation
                        NeuronType::Inhibitory => -20.0, // Strong inhibition
                        NeuronType::Pacemaker => 30.0, // Strong drive
                    };
                    brain.add_synapse(i, j, weight + rng.gen_range(-5.0..5.0));
                }
            } else if rng.gen::<f32>() < 0.005 {
                // Long range
                let weight = match brain.neurons[i].neuron_type {
                    NeuronType::Excitatory => 10.0,
                    NeuronType::Inhibitory => -15.0,
                    NeuronType::Pacemaker => 20.0,
                };
                brain.add_synapse(i, j, weight);
            }
        }
    }

    let mut metronome_timer = 0.0;
    let metronome_interval = 0.5; // 120 BPM
    let mut use_metronome = true;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        let dt = get_frame_time().min(0.05); // Cap dt

        if is_key_pressed(KeyCode::Space) {
            use_metronome = !use_metronome;
        }

        // Metronome
        let mut pulse = false;
        if use_metronome {
            metronome_timer += dt;
            if metronome_timer >= metronome_interval {
                metronome_timer = 0.0;
                pulse = true;
                // Play kick
                 let _ = cmd_tx.send(AudioCommand::Trigger {
                    frequency: 60.0,
                    decay: 0.8,
                    amplitude: 0.8,
                    waveform: Waveform::Sine, // Kick-ish
                });
            }
        }

        // Prepare External Currents
        let mut external_currents = vec![0.0; brain.neurons.len()];
        let (mx, my) = mouse_position();
        let m_pos = vec2(mx, my);
        let mouse_active = is_mouse_button_down(MouseButton::Left);

        for (i, neuron) in brain.neurons.iter().enumerate() {
            let mut current = 0.0;

            // Mouse Influence
            if mouse_active {
                let d = neuron.pos.distance(m_pos);
                if d < 100.0 {
                    current += 50.0 * (1.0 - d / 100.0);
                }
            }

            // Metronome Influence (Pacemakers)
            if pulse && neuron.neuron_type == NeuronType::Pacemaker {
                current += 100.0; // Strong kick
            }

            // Random noise (Chaos)
            if rng.gen::<f32>() < 0.01 {
                current += 20.0;
            }

            external_currents[i] = current;
        }

        // Update Brain
        brain.update(dt * 1000.0, &external_currents); // dt in seconds, Izhikevich uses roughly ms?
        // Wait, Izhikevich model: "t" is usually ms.
        // If dt is 0.016s (16ms), we should pass 16.0?
        // Yes, step is usually 1ms or 0.1ms.
        // If we pass 0.016, it will be very slow.
        // Let's pass dt * 1000.0.

        // Audio Triggering
        for &spike_idx in &brain.spikes {
            let neuron = &brain.neurons[spike_idx];

            // Map position to frequency
            // Map Y to note index
            let y_norm = (neuron.pos.y - start_y) / (GRID_ROWS as f32 * SPACING);
            let note_idx = ((1.0 - y_norm) * 15.0) as usize; // 15 notes range
            let freq = get_pentatonic_freq(note_idx);

            match neuron.neuron_type {
                NeuronType::Excitatory => {
                    let _ = cmd_tx.send(AudioCommand::Trigger {
                        frequency: freq,
                        decay: 0.95,
                        amplitude: 0.1,
                        waveform: Waveform::Sine,
                    });
                },
                NeuronType::Inhibitory => {
                    // Hi-hats or clicks?
                     let _ = cmd_tx.send(AudioCommand::Trigger {
                        frequency: freq * 4.0, // High pitch
                        decay: 0.5,
                        amplitude: 0.05,
                        waveform: Waveform::Noise,
                    });
                },
                NeuronType::Pacemaker => {
                    // Snare?
                     let _ = cmd_tx.send(AudioCommand::Trigger {
                        frequency: 150.0,
                        decay: 0.7,
                        amplitude: 0.3,
                        waveform: Waveform::Square,
                    });
                }
            }
        }

        // Visualization
        for neuron in &brain.neurons {
            let color = if neuron.spiked {
                WHITE
            } else {
                // Map voltage to brightness
                let v_norm = ((neuron.voltage + 80.0) / 100.0).clamp(0.0, 1.0);
                match neuron.neuron_type {
                    NeuronType::Excitatory => Color::new(0.2, 0.2, v_norm, 1.0), // Blue
                    NeuronType::Inhibitory => Color::new(v_norm, 0.2, 0.2, 1.0), // Red
                    NeuronType::Pacemaker => Color::new(0.2, v_norm, 0.2, 1.0), // Green
                }
            };

            draw_circle(neuron.pos.x, neuron.pos.y, NEURON_RADIUS, color);

            // Draw connections for spiked neurons? Too many.
            // Maybe just draw lines for Pacemakers to neighbors
            if neuron.neuron_type == NeuronType::Pacemaker {
                // Visualize pacemaker specific things if needed
            }
        }

        // Draw spiked connections (flashes)
        for &spike_idx in &brain.spikes {
            let neuron = &brain.neurons[spike_idx];
            // We can't see outgoing easily without O(N) search or storing outgoing.
            // Let's just draw a ring.
             draw_circle_lines(neuron.pos.x, neuron.pos.y, NEURON_RADIUS * 3.0, 1.0, WHITE);
        }

        draw_text("Neural Beatbox", 10.0, 30.0, 30.0, WHITE);
        draw_text(if use_metronome { "Metronome: ON (Space)" } else { "Metronome: OFF (Space)" }, 10.0, 50.0, 20.0, GRAY);
        draw_text("Click to stimulate", 10.0, 70.0, 20.0, GRAY);

        next_frame().await;
    }
}
