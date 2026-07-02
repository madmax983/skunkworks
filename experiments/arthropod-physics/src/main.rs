use arthropod::Button;
use macroquad::prelude::*;
use physics_pbd::glam::Vec3;
use physics_pbd::{Constraint, PbdSystem};

// By creating a custom main wrapper, we bypass the macroquad window initialization entirely for headless.
// The standard #[macroquad::main] macro initializes X11 *before* our function code runs.
fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Physics".to_owned(),
        ..Default::default()
    }
}

async fn run_sim() {
    let mut system = PbdSystem::new();
    let num_particles = 10;

    // Create a chain of particles
    let mut prev_idx = None;
    for i in 0..num_particles {
        let pos = Vec3::new(screen_width() / 2.0 + (i as f32 * 20.0), 100.0, 0.0);
        let mass = if i == 0 { 0.0 } else { 1.0 }; // Pin the first particle
        let idx = system.add_particle(pos, mass);

        if let Some(p) = prev_idx {
            let _ = system.add_distance_constraint(p, idx, 20.0);
        }
        prev_idx = Some(idx);
    }

    let explode_btn =
        Button::new("Explode", 10.0, 10.0, 150.0, 40.0).with_colors(RED, ORANGE, DARKGRAY);

    let add_btn =
        Button::new("Add Particle", 10.0, 60.0, 150.0, 40.0).with_colors(BLUE, SKYBLUE, DARKGRAY);

    loop {
        clear_background(BLACK);

        // Gravity
        for p in &mut system.particles {
            if p.inv_mass > 0.0 {
                p.vel.y += 9.8 * 0.1;
            }
        }

        system.step(0.1, 5);

        if explode_btn.draw() {
            for p in &mut system.particles {
                if p.inv_mass > 0.0 {
                    p.vel = Vec3::new(
                        rand::gen_range(-100.0, 100.0),
                        rand::gen_range(-100.0, 100.0),
                        0.0,
                    );
                }
            }
        }

        if add_btn.draw() {
            let pos = Vec3::new(mouse_position().0, mouse_position().1, 0.0);
            let idx = system.add_particle(pos, 1.0);
            if let Some(last) = prev_idx {
                let _ = system.add_distance_constraint(last, idx, 20.0);
            }
            prev_idx = Some(idx);
        }

        // Draw constraints
        for c in &system.constraints {
            match c {
                Constraint::Distance { p1, p2, .. } | Constraint::Actuator { p1, p2, .. } => {
                    let pos1 = system.particles[*p1].pos;
                    let pos2 = system.particles[*p2].pos;
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0, WHITE);
                }
                _ => {}
            }
        }

        // Draw particles
        for p in &system.particles {
            draw_circle(
                p.pos.x,
                p.pos.y,
                5.0,
                if p.inv_mass == 0.0 { RED } else { GREEN },
            );
        }

        next_frame().await
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from("--headless")) {
        println!("Headless mode: exiting early to prevent X11 panics or execution timeouts.");
        return;
    }
    macroquad::Window::from_config(window_conf(), run_sim());
}
