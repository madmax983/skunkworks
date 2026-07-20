//! # Arthropod Locus 📍🐜
//!
//! **Concept:** Interactive Topological Manipulation.
//!
//! This hybrid visualizer explores crossing the immediate-mode UI library of `arthropod` with the topological boundary definitions of `locus`.
//!
//! ## Lineage
//! - **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons.
//! - **Parent B (crates/locus):** Provides the topology (Torus, Klein Bottle, etc.) defining boundary wrapping.
//!
//! ## Novel Trait
//! A discrete, interactive simulation where users spawn entities and can dynamically change the boundary rules (the `Topology`) driving their movement. Abstract GUI clicks seamlessly alter the continuous wrapping space the entities experience.
//!
//! ## Predicted Phenotype
//! An interactive laboratory for exploring non-Euclidean boundaries. By clicking buttons to change the `Topology` from `Torus` to `Klein` to `Mobius`, the visual flow of moving entities instantly transforms as their coordinate wrapping rules shift.
//!
//! ## Usage
//!
//! ```sh
//! cargo run -p arthropod-locus --release
//! ```
//!
//! *(Note: Use `cargo run -p arthropod-locus --release -- --headless` to safely bypass X11 UI panics in CI environments.)*

use arthropod::Button;
use locus::{Topology, Vec2};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Locus".to_owned(),
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

struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: Color,
}

async fn async_main() {
    let mut current_topo = Topology::Torus;

    let w = 800;
    let h = 600;

    let mut particles = Vec::new();
    for i in 0..100 {
        particles.push(Particle {
            pos: Vec2::new((i * 15 % w) as f64, (i * 10 % h) as f64),
            vel: Vec2::new(2.0, 1.5),
            color: WHITE,
        });
    }

    let btn_torus = Button::new("Torus", 20.0, 20.0, 100.0, 40.0).with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_klein = Button::new("Klein", 20.0, 70.0, 100.0, 40.0).with_colors(GREEN, LIME, DARKGREEN);
    let btn_mobius = Button::new("Mobius", 20.0, 120.0, 100.0, 40.0).with_colors(RED, ORANGE, MAROON);
    let btn_plane = Button::new("Plane", 20.0, 170.0, 100.0, 40.0).with_colors(GRAY, LIGHTGRAY, BLACK);

    loop {
        clear_background(color_u8!(20, 20, 30, 255));

        if btn_torus.draw() { current_topo = Topology::Torus; }
        if btn_klein.draw() { current_topo = Topology::Klein; }
        if btn_mobius.draw() { current_topo = Topology::Mobius; }
        if btn_plane.draw() { current_topo = Topology::Plane; }

        draw_text(&format!("Current Topology: {:?}", current_topo), 20.0, 250.0, 20.0, WHITE);

        for p in &mut particles {
            p.pos += p.vel;

            let y_idx = p.pos.y.round() as i64;
            let x_idx = p.pos.x.round() as i64;

            if let Some((ny, nx)) = current_topo.normalize(y_idx, x_idx, w as usize, h as usize) {
                p.pos.y = ny as f64;
                p.pos.x = nx as f64;
            } else {
                // If it goes out of bounds and doesn't wrap, we bounce it to keep it visible
                if p.pos.x < 0.0 || p.pos.x > w as f64 { p.vel.x *= -1.0; }
                if p.pos.y < 0.0 || p.pos.y > h as f64 { p.vel.y *= -1.0; }

                // Clamp
                p.pos.x = p.pos.x.clamp(0.0, w as f64);
                p.pos.y = p.pos.y.clamp(0.0, h as f64);
            }

            draw_circle(p.pos.x as f32, p.pos.y as f32, 4.0, p.color);
        }

        next_frame().await;
    }
}
