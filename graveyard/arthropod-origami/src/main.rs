//! # Arthropod Origami
//!
//! This hybrid bridges the abstract immediate mode UI paradigms of `arthropod` with the continuous soft-body mesh generation of `origami`. It allows the user to interactively fold, expand, and contract the Miura-ori tessellation directly via GUI controls.
//!
//! ## Quick Start
//!
//! ```sh
//! # Run interactively (GUI)
//! cargo run -p arthropod-origami --release
//!
//! # Run headlessly (CI/Non-interactive)
//! cargo run -p arthropod-origami --release -- --headless
//! ```

use arthropod::Button;
use macroquad::prelude::*;
use origami::{generate_miura_mesh, MiuraParams, Orientation};

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Origami".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

async fn run() {
    let mut camera = Camera3D {
        position: vec3(0.0, 50.0, 100.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let mut extension_factor = 0.5;

    let btn_expand =
        Button::new("Expand", 10.0, 10.0, 150.0, 30.0).with_colors(BLUE, SKYBLUE, DARKBLUE);

    let btn_contract =
        Button::new("Contract", 10.0, 50.0, 150.0, 30.0).with_colors(RED, ORANGE, MAROON);

    let mut rotation: f32 = 0.0;

    loop {
        clear_background(BLACK);

        rotation += 0.01;
        camera.position = vec3(rotation.sin() * 40.0, 30.0, rotation.cos() * 40.0);

        set_camera(&camera);

        let params = MiuraParams {
            a: 2.0,
            b: 2.0,
            gamma: 1.4,
            orientation: Orientation::Horizontal,
        };

        let mesh = generate_miura_mesh(params, (20, 20), extension_factor);

        let offset = vec3(-20.0, 0.0, -20.0);

        for chunk in mesh.indices.chunks(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let p0 = mesh.vertices[i0].pos + offset;
            let p1 = mesh.vertices[i1].pos + offset;
            let p2 = mesh.vertices[i2].pos + offset;

            // Draw wireframe for the paper folds
            draw_line_3d(p0, p1, WHITE);
            draw_line_3d(p1, p2, WHITE);
            draw_line_3d(p2, p0, WHITE);
        }

        set_default_camera();

        // UI Layer
        if btn_expand.draw() {
            extension_factor = (extension_factor + 0.05).min(1.0);
        }
        if btn_contract.draw() {
            extension_factor = (extension_factor - 0.05).max(0.01);
        }

        draw_text(
            &format!("Extension: {:.2}", extension_factor),
            10.0,
            110.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Exiting immediately.");
        return;
    }

    macroquad::Window::from_config(window_conf(), run());
}
