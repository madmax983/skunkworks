mod audio;
mod network;
mod neuron;

use crate::audio::{AudioEngine, Command, Snapshot};
use crate::network::Network;
use macroquad::prelude::*;
use std::collections::VecDeque;
use std::thread;

const HISTORY_LEN: usize = 600;

#[macroquad::main("Biomimetic Synth")]
async fn main() {
    // 1. Initialize Network
    let num_neurons = 100;
    let mut net = Network::new(num_neurons);

    // Connect them somewhat randomly
    // 10% connectivity, weight up to 15.0
    net.connect_random(0.1, 15.0);

    // Initialize Audio Engine
    let (engine, cmd_tx, snap_rx) = AudioEngine::new(net);

    // Spawn Audio Thread
    thread::spawn(move || {
        engine.run();
    });

    // UI State
    let mut mean_field_history: VecDeque<f32> = VecDeque::with_capacity(HISTORY_LEN);
    for _ in 0..HISTORY_LEN {
        mean_field_history.push_back(0.0);
    }

    let mut last_snapshot = Snapshot {
        voltages: vec![-65.0; num_neurons],
        spikes: vec![false; num_neurons],
        synapses: Vec::new(),
        mean_field: -65.0,
    };

    let grid_size = (num_neurons as f32).sqrt().ceil() as usize;

    loop {
        let sw = screen_width();
        let sh = screen_height();

        // 2. Process Input
        let (mx, my) = mouse_position();

        // Grid Calculation
        let grid_w = sw; // Full width
        let grid_h = sh * 0.7; // Top 70%
        let cell_w = grid_w / grid_size as f32;
        let cell_h = grid_h / grid_size as f32;

        if is_mouse_button_down(MouseButton::Left) {
            // Find neuron under mouse
            if my < grid_h {
                let col = (mx / cell_w) as usize;
                let row = (my / cell_h) as usize;
                let index = row * grid_size + col;

                if index < num_neurons {
                    // Inject current
                    let _ = cmd_tx.send(Command::Inject {
                        index,
                        current: 20.0,
                    });
                }
            }
        }

        if is_key_pressed(KeyCode::R) {
            for i in 0..num_neurons {
                let _ = cmd_tx.send(Command::SetParams {
                    index: i,
                    a: 0.02,
                    b: 0.2,
                    c: -50.0,
                    d: 2.0,
                });
            }
        }

        // 3. Receive Data
        while let Ok(snap) = snap_rx.try_recv() {
            last_snapshot = snap;
            mean_field_history.push_back(last_snapshot.mean_field);
            if mean_field_history.len() > HISTORY_LEN {
                mean_field_history.pop_front();
            }
        }

        // 4. Render
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0)); // Dark Gray

        // Draw Synapses first (background layer)
        for syn in &last_snapshot.synapses {
            let pre = syn.pre;
            let post = syn.post;
            let weight = syn.weight;

            let c1 = pre % grid_size;
            let r1 = pre / grid_size;
            let x1 = c1 as f32 * cell_w + cell_w * 0.5;
            let y1 = r1 as f32 * cell_h + cell_h * 0.5;

            let c2 = post % grid_size;
            let r2 = post / grid_size;
            let x2 = c2 as f32 * cell_w + cell_w * 0.5;
            let y2 = r2 as f32 * cell_h + cell_h * 0.5;

            let alpha = (weight / 20.0).clamp(0.05, 0.4);
            draw_line(x1, y1, x2, y2, 1.0, Color::new(0.4, 0.4, 0.4, alpha));
        }

        // Draw Neurons
        for i in 0..num_neurons {
            let v = last_snapshot.voltages[i];
            let spiked = last_snapshot.spikes[i];

            let col = i % grid_size;
            let row = i / grid_size;

            let x = col as f32 * cell_w;
            let y = row as f32 * cell_h;

            // Color based on voltage
            let color = if spiked {
                WHITE
            } else if v < -60.0 {
                let t = ((v + 80.0) / 20.0).clamp(0.0, 1.0);
                Color::new(0.0, 0.0, t.max(0.2), 1.0)
            } else if v < -40.0 {
                let t = ((v + 60.0) / 20.0).clamp(0.0, 1.0);
                Color::new(0.0, t, 1.0 - t, 1.0)
            } else {
                let t = ((v + 40.0) / 70.0).clamp(0.0, 1.0);
                Color::new(t, 1.0 - t * 0.5, 0.0, 1.0)
            };

            draw_rectangle(x + 2.0, y + 2.0, cell_w - 4.0, cell_h - 4.0, color);

            if spiked {
                draw_rectangle_lines(x, y, cell_w, cell_h, 2.0, YELLOW);
            }
        }

        // Draw Waveform
        let wave_y = sh * 0.85;
        draw_rectangle(0.0, sh * 0.7, sw, sh * 0.3, BLACK);
        draw_line(0.0, wave_y, sw, wave_y, 1.0, DARKGRAY);

        let step_x = sw / HISTORY_LEN as f32;

        for i in 0..mean_field_history.len() - 1 {
            let v1 = mean_field_history[i];
            let v2 = mean_field_history[i + 1];

            // Normalize for visualization
            let y1 = wave_y - (v1 * 500.0); // Amplified
            let y2 = wave_y - (v2 * 500.0);

            draw_line(
                i as f32 * step_x,
                y1,
                (i + 1) as f32 * step_x,
                y2,
                2.0,
                GREEN,
            );
        }

        // Info Text
        draw_text("BIOMIMETIC SYNTH", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            "Click to Stimulate | R: Set to Chattering",
            10.0,
            sh - 20.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
