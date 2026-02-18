use macroquad::prelude::*;

mod circuit_gen;
mod neuro;

use circuit_gen::{Circuit, CircuitGenerator};
use neuro::NeuralCircuit;

#[macroquad::main("NeuroCircuit")]
async fn main() {
    let width = 800;
    let height = 600;

    request_new_screen_size(width as f32, height as f32);

    let mut seed_counter = 0;
    let mut circuit = generate_circuit(width, height, seed_counter);
    let mut brain = NeuralCircuit::new(circuit.pads.len());

    loop {
        // Input
        if is_key_pressed(KeyCode::R) {
            seed_counter += 1;
            circuit = generate_circuit(width, height, seed_counter);
            brain = NeuralCircuit::new(circuit.pads.len());
        }

        // Update
        // Run physics. Izhikevich models usually expect dt=1ms or 0.5ms.
        // We simulate "time" here.
        brain.update(0.5, &circuit);

        // Draw
        clear_background(Color::new(0.0, 0.1, 0.05, 1.0)); // Dark PCB Green

        // Draw Traces
        for trace in &circuit.traces {
            if trace.path.len() < 2 { continue; }
            for i in 0..trace.path.len() - 1 {
                let (x1, y1) = trace.path[i];
                let (x2, y2) = trace.path[i+1];
                draw_line(x1 as f32, y1 as f32, x2 as f32, y2 as f32, 2.0, Color::new(0.0, 0.4, 0.0, 1.0));
            }
        }

        // Draw Pads
        for (i, &(px, py)) in circuit.pads.iter().enumerate() {
            let neuron = &brain.neurons[i];

            // Visual voltage mapping
            // Resting is approx -65. Spiking is 30.
            // Map -80..30 to color

            let v_norm = (neuron.v + 80.0) / 110.0;
            let t = v_norm.clamp(0.0, 1.0);

            let color = if neuron.v >= 20.0 {
                WHITE // Flash
            } else {
                Color::new(0.8 * t, 0.6 * t, 0.1, 1.0) // Gold/Orange variable
            };

            draw_circle(px as f32, py as f32, 6.0, color);
            draw_circle_lines(px as f32, py as f32, 6.0, 1.0, BLACK); // Hole
        }

        // Draw Pulses
        for pulse in &brain.pulses {
            let trace = &circuit.traces[pulse.trace_idx];
            if pulse.position_idx < trace.path.len() {
                let (x, y) = trace.path[pulse.position_idx];
                draw_circle(x as f32, y as f32, 3.0, WHITE);
                // Glow
                draw_circle(x as f32, y as f32, 6.0, Color::new(1.0, 1.0, 1.0, 0.3));
            }
        }

        // UI
        draw_text("NEURO-CIRCUIT", 20.0, 30.0, 30.0, WHITE);
        draw_text(format!("Neurons: {}", brain.neurons.len()).as_str(), 20.0, 50.0, 20.0, GRAY);
        draw_text(format!("Pulses: {}", brain.pulses.len()).as_str(), 20.0, 70.0, 20.0, GRAY);
        draw_text("Press [R] to Regenerate", 20.0, 90.0, 20.0, GRAY);

        next_frame().await
    }
}

fn generate_circuit(w: u32, h: u32, seed: u64) -> Circuit {
    let gen = CircuitGenerator::new(w, h);
    gen.generate(&seed.to_string())
}
