//! # arthropod-physics 🧬
//!
//! **Lineage**: `arthropod` (Immediate-mode GUI) × `physics-pbd` (Position Based Dynamics)
//!
//! ## The Concept
//! This hybrid bridges the abstract interface paradigm with structural physical rendering. We inject discrete GUI inputs from `arthropod` directly into the continuous soft-body constraints of `physics-pbd`. The UI buttons allow the user to dynamically actuate constraints (like expanding or contracting muscles) or apply kinetic forces to a hanging physical mesh, observing how structural rigidity reacts to external abstract controls.
//!
//! ## Phenotype
//! An interactive structural laboratory where abstract button clicks physically yank, stretch, and deform a continuous soft-body mesh hanging in a simulated gravity well.
//!
//! ## Execution
//! Run this experiment using:
//! ```bash
//! cargo run -p arthropod-physics
//! ```
//!
//! For headless validation (e.g. CI environments), run:
//! ```bash
//! cargo run -p arthropod-physics -- --headless
//! ```
//!
use arthropod::Button;
use macroquad::prelude::*;
use physics_pbd::{Constraint, PbdSystem};

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod × Physics PBD".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode. Exiting early to avoid X11 panic.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut system = PbdSystem::new();

    // Create a hanging soft body
    let w = 10;
    let h = 10;
    let spacing = 1.0;

    let mut particles = vec![];
    for y in 0..h {
        for x in 0..w {
            let px = x as f32 * spacing - (w as f32 * spacing) / 2.0;
            let py = -(y as f32) * spacing;

            // Top row is anchored (mass 0.0)
            let mass = if y == 0 { 0.0 } else { 1.0 };
            let id = system.add_particle(physics_pbd::glam::Vec3::new(px, py, 0.0), mass);
            particles.push(id);
        }
    }

    // Connect them
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;

            // Right
            if x < w - 1 {
                system
                    .add_distance_constraint(particles[i], particles[i + 1], spacing)
                    .unwrap();
            }
            // Down
            if y < h - 1 {
                system
                    .add_distance_constraint(particles[i], particles[i + w], spacing)
                    .unwrap();
            }
        }
    }

    let btn_push = Button::new("Push", 20.0, 20.0, 100.0, 40.0).with_colors(RED, ORANGE, YELLOW);

    let btn_pull = Button::new("Pull", 20.0, 70.0, 100.0, 40.0).with_colors(
        BLUE,
        Color::new(0.5, 0.5, 1.0, 1.0),
        WHITE,
    );

    loop {
        clear_background(BLACK);

        // Add some gravity
        for i in 0..system.particles.len() {
            if system.particles[i].inv_mass > 0.0 {
                system.particles[i].vel += physics_pbd::glam::Vec3::new(0.0, -9.8 * 0.016, 0.0);
            }
        }

        system.step(0.016, 5);

        // Render
        set_camera(&Camera3D {
            position: vec3(0., -5., 15.),
            up: vec3(0., 1., 0.),
            target: vec3(0., -5., 0.),
            ..Default::default()
        });

        for c in &system.constraints {
            if let Constraint::Distance { p1, p2, .. } = c {
                let p1_pos = system.particles[*p1].pos;
                let p2_pos = system.particles[*p2].pos;
                draw_line_3d(
                    vec3(p1_pos.x, p1_pos.y, p1_pos.z),
                    vec3(p2_pos.x, p2_pos.y, p2_pos.z),
                    Color::new(0.5, 0.5, 1.0, 1.0),
                );
            }
        }

        for p in &system.particles {
            draw_sphere(
                vec3(p.pos.x, p.pos.y, p.pos.z),
                0.1,
                None,
                Color::new(1.0, 1.0, 1.0, 1.0),
            );
        }

        set_default_camera();

        // Draw UI after 3D scene (in immediate mode GUI, doing logic inside the if statement and then not drawing later prevents double-drawing/double-input)
        let push_pressed = btn_push.draw();
        let pull_pressed = btn_pull.draw();

        if push_pressed {
            for i in 0..system.particles.len() {
                if system.particles[i].inv_mass > 0.0 {
                    system.particles[i].vel += physics_pbd::glam::Vec3::new(0.0, 0.0, -10.0);
                }
            }
        }

        if pull_pressed {
            for i in 0..system.particles.len() {
                if system.particles[i].inv_mass > 0.0 {
                    system.particles[i].vel += physics_pbd::glam::Vec3::new(0.0, 10.0, 0.0);
                }
            }
        }

        next_frame().await;
    }
}
