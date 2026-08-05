//! # hyper-neuro
//!
//! **A Splice Surgeon Hybrid**
//!
//! ## Concept
//! Hyper-dimensional Neural Morphogenesis.
//!
//! ## Lineage
//! This experiment is a direct cross between:
//! 1. **`crates/hyper-system`**: Provides the 4D vector mathematics (`Vec4`) and perspective projection logic required to render a rotating hypercube.
//! 2. **`crates/neuro-sim`**: Provides the continuous biological simulation of an Izhikevich Spiking Neural Network (`Network`).
//!
//! ## Phenotype
//! The continuous 4D rotation parameters of the hypercube are driven entirely by the biological Spiking Neural Network. As neurons spike, they impart discrete "thoughts" or angular momentum to rotate the 4D projection planes (XW, YW, ZW). We are visually mapping cognitive load into 4D coordinate rotation.
//!
//! ## Execution
//! Run with `cargo run -p hyper-neuro`.
//!
//! **CI Safety**: Supports `--headless` flag to bypass the macroquad UI loop and prevent X11 panics in headless environments.
//!
use hyper_system::Vec4;
use macroquad::prelude::*;
use neuro_sim::Network;

fn window_conf() -> Conf {
    Conf {
        window_title: "Hyper Neuro".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Ensure the headless bypass is checked synchronously before `Window::from_config`
fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode enabled. Bypassing macroquad initialization.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut brain = Network::new();
    let n1 = brain.add_neuron();
    let n2 = brain.add_neuron();
    let n3 = brain.add_neuron();

    // Setup an oscillator loop
    brain.add_synapse(n1, n2, 10.0);
    brain.add_synapse(n2, n3, 10.0);
    brain.add_synapse(n3, n1, 10.0);

    let mut rot_xw_angle = 0.0;
    let mut rot_yw_angle = 0.0;
    let mut rot_zw_angle = 0.0;

    let points = vec![
        Vec4::new(-1.0, -1.0, -1.0, -1.0),
        Vec4::new(1.0, -1.0, -1.0, -1.0),
        Vec4::new(1.0, 1.0, -1.0, -1.0),
        Vec4::new(-1.0, 1.0, -1.0, -1.0),
        Vec4::new(-1.0, -1.0, 1.0, -1.0),
        Vec4::new(1.0, -1.0, 1.0, -1.0),
        Vec4::new(1.0, 1.0, 1.0, -1.0),
        Vec4::new(-1.0, 1.0, 1.0, -1.0),
        Vec4::new(-1.0, -1.0, -1.0, 1.0),
        Vec4::new(1.0, -1.0, -1.0, 1.0),
        Vec4::new(1.0, 1.0, -1.0, 1.0),
        Vec4::new(-1.0, 1.0, -1.0, 1.0),
        Vec4::new(-1.0, -1.0, 1.0, 1.0),
        Vec4::new(1.0, -1.0, 1.0, 1.0),
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        Vec4::new(-1.0, 1.0, 1.0, 1.0),
    ];

    loop {
        clear_background(BLACK);

        // Inject random noise to stimulate the network
        let mut inputs = vec![0.0; 3];
        if rand::gen_range(0.0, 1.0) < 0.05 {
            inputs[0] = 50.0;
        }

        brain.step(&inputs);

        // Read spikes to actuate rotation
        if brain.is_spiking(n1) {
            rot_xw_angle += 0.1;
        }
        if brain.is_spiking(n2) {
            rot_yw_angle += 0.1;
        }
        if brain.is_spiking(n3) {
            rot_zw_angle += 0.1;
        }

        // Apply a small constant rotation just to keep things slightly drifting
        rot_xw_angle += 0.001;
        rot_yw_angle += 0.002;
        rot_zw_angle += 0.0015;

        // Draw hypercube
        for p in &points {
            let p_rot = p
                .rotate_xw(rot_xw_angle)
                .rotate_yw(rot_yw_angle)
                .rotate_zw(rot_zw_angle);
            // Camera dist 3.0
            let p3 = p_rot.project_to_3d(3.0);

            // Map 3D to 2D screen
            let screen_x = p3.x * 100.0 + screen_width() / 2.0;
            let screen_y = p3.y * 100.0 + screen_height() / 2.0;

            draw_circle(screen_x, screen_y, 4.0, RED);
        }

        // Connect the vertices of the hypercube
        let edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
            (8, 9),
            (9, 10),
            (10, 11),
            (11, 8),
            (12, 13),
            (13, 14),
            (14, 15),
            (15, 12),
            (8, 12),
            (9, 13),
            (10, 14),
            (11, 15),
            (0, 8),
            (1, 9),
            (2, 10),
            (3, 11),
            (4, 12),
            (5, 13),
            (6, 14),
            (7, 15),
        ];

        for &(i, j) in &edges {
            let p1 = points[i]
                .rotate_xw(rot_xw_angle)
                .rotate_yw(rot_yw_angle)
                .rotate_zw(rot_zw_angle)
                .project_to_3d(3.0);
            let p2 = points[j]
                .rotate_xw(rot_xw_angle)
                .rotate_yw(rot_yw_angle)
                .rotate_zw(rot_zw_angle)
                .project_to_3d(3.0);

            let sx1 = p1.x * 100.0 + screen_width() / 2.0;
            let sy1 = p1.y * 100.0 + screen_height() / 2.0;
            let sx2 = p2.x * 100.0 + screen_width() / 2.0;
            let sy2 = p2.y * 100.0 + screen_height() / 2.0;

            draw_line(sx1, sy1, sx2, sy2, 1.0, WHITE);
        }

        draw_text(
            "Hyper-dimensional Neural Morphogenesis",
            10.0,
            20.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!(
                "Neurons Spiking: n1:{} n2:{} n3:{}",
                brain.is_spiking(n1),
                brain.is_spiking(n2),
                brain.is_spiking(n3)
            ),
            10.0,
            40.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}
