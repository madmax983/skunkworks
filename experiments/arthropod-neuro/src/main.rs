//! # Arthropod Neuro 🐜🧠
//!
//! **Concept:** Interactive Neural Stimulation.
//!
//! This hybrid visualizer crosses the immediate-mode UI library of `arthropod`
//! with the continuous biological Spiking Neural Network of `neuro-sim`.
//! It allows users to physically "poke" neurons via UI clicks to observe
//! biological wave propagation through the network.

use arthropod::Button;
use macroquad::prelude::*;
use neuro_sim::Network;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Neuro".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing macroquad initialization.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut network = Network::new();
    let n1 = network.add_neuron();
    let n2 = network.add_neuron();
    let n3 = network.add_neuron();

    // n1 excites n2, n2 excites n3
    network.add_synapse(n1, n2, 30.0);
    network.add_synapse(n2, n3, 30.0);

    let btn_poke = Button::new("Poke Neuron 1", 20.0, 20.0, 150.0, 40.0)
        .with_colors(RED, ORANGE, DARKGRAY);

    loop {
        clear_background(color_u8!(20, 20, 30, 255)); // dark slate

        let mut external_inputs = vec![0.0, 0.0, 0.0];

        if btn_poke.draw() {
            external_inputs[0] = 50.0; // Inject strong current into n1
        }

        // Step the network
        network.step(&external_inputs);

        // Draw neurons
        let neuron_positions = [
            (200.0, 300.0),
            (400.0, 300.0),
            (600.0, 300.0),
        ];

        for i in 0..3 {
            let (x, y) = neuron_positions[i];
            let v = network.neurons[i].v;

            // Map voltage (-65 to 30) to a color (blue to red)
            let mut r = ((v + 65.0) / 95.0 * 255.0).clamp(0.0, 255.0) as u8;
            let mut b = 255 - r;

            // Highlight spikes
            if network.is_spiking(i) {
                r = 255;
                b = 255; // Flash white
            }

            draw_circle(x, y, 20.0, color_u8!(r, 50, b, 255));
            draw_text(&format!("N{}: {:.1}mV", i+1, v), x - 20.0, y - 30.0, 20.0, WHITE);
        }

        // Draw synapses
        for i in 0..2 {
            let (x1, y1) = neuron_positions[i];
            let (x2, y2) = neuron_positions[i+1];

            let is_active = network.get_synapse_activity(i);
            let col = if is_active { YELLOW } else { GRAY };
            let thickness = if is_active { 5.0 } else { 2.0 };

            draw_line(x1 + 20.0, y1, x2 - 20.0, y2, thickness, col);
        }

        next_frame().await;
    }
}
