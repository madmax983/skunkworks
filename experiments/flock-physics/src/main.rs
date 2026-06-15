use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use macroquad::prelude::*;
use physics_pbd::{Constraint, PbdSystem};
use std::env;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "flock-physics".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--headless") {
        println!("Headless mode: exiting early to prevent X11 panics or execution timeouts.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut system = PbdSystem::new();

    // Create a 10x10 soft body mesh
    let rows = 10;
    let cols = 10;
    let spacing = 20.0;
    let offset_x = (WIDTH as f32 - (cols as f32 * spacing)) / 2.0;
    let offset_y = (HEIGHT as f32 - (rows as f32 * spacing)) / 2.0;

    let mut p_indices = vec![];
    for y in 0..rows {
        for x in 0..cols {
            let px = offset_x + x as f32 * spacing;
            let py = offset_y + y as f32 * spacing;
            // Pin the top corners
            let inv_mass = if y == 0 && (x == 0 || x == cols - 1) { 0.0 } else { 1.0 };
            p_indices.push(system.add_particle(::glam::Vec3::new(px, py, 0.0), inv_mass));
        }
    }

    // Add structural constraints
    for y in 0..rows {
        for x in 0..cols {
            let idx = y * cols + x;
            if x < cols - 1 {
                let _ = system.add_distance_constraint(p_indices[idx], p_indices[idx + 1], spacing);
            }
            if y < rows - 1 {
                let _ = system.add_distance_constraint(p_indices[idx], p_indices[idx + cols], spacing);
            }
        }
    }

    // Initialize the flock
    let num_boids = 50;
    let mut positions: Vec<Vec2> = (0..num_boids)
        .map(|_| {
            Vec2::new(
                rand::gen_range(0.0, WIDTH),
                rand::gen_range(0.0, HEIGHT),
            )
        })
        .collect();
    let mut velocities: Vec<Vec2> = (0..num_boids)
        .map(|_| {
            Vec2::new(
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
            )
            .normalize()
                * 2.0
        })
        .collect();

    let flock_params = FlockingParams {
        view_radius: 50.0,
        separation_radius: 20.0,
        max_speed: 4.0,
        max_force: 0.1,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    loop {
        clear_background(BLACK);

        // --- 1. Update Flock ---
        let forces: Vec<Vec2> = (0..num_boids)
            .map(|i| compute_force(&positions, &velocities, i, &flock_params))
            .collect();

        for i in 0..num_boids {
            velocities[i] += forces[i];
            if velocities[i].magnitude() > flock_params.max_speed {
                velocities[i] = velocities[i].normalize() * flock_params.max_speed;
            }
            positions[i] += velocities[i];

            // Wrap around screen
            if positions[i].x < 0.0 {
                positions[i].x += WIDTH;
            }
            if positions[i].x > WIDTH {
                positions[i].x -= WIDTH;
            }
            if positions[i].y < 0.0 {
                positions[i].y += HEIGHT;
            }
            if positions[i].y > HEIGHT {
                positions[i].y -= HEIGHT;
            }
        }

        // --- 2. Flocking influences Physics ---
        // For each boid, find nearby soft body particles and apply a force
        let interaction_radius = 40.0;
        let force_multiplier = 0.5;

        for i in 0..num_boids {
            let boid_pos = positions[i];
            for p_idx in 0..system.particles.len() {
                let p_pos = system.particles[p_idx].pos;
                let dist = Vec2::new(p_pos.x as f64, p_pos.y as f64).distance(boid_pos);

                if dist < interaction_radius && system.particles[p_idx].inv_mass > 0.0 {
                    // Push particle along boid velocity
                    let push = ::glam::Vec3::new(velocities[i].x as f32, velocities[i].y as f32, 0.0) * force_multiplier;
                    system.particles[p_idx].vel += push;
                }
            }
        }

        // --- 3. Update Physics ---
        // Add gravity
        for p in system.particles.iter_mut() {
            if p.inv_mass > 0.0 {
                p.vel.y += 9.8 * 0.016; // Gravity
            }
        }

        system.step(0.016, 5); // 5 solver iterations

        // --- 4. Render Physics ---
        // Render constraints (bonds)
        for c in &system.constraints {
            match c {
                Constraint::Distance { p1, p2, .. } => {
                    let pos1 = system.particles[*p1].pos;
                    let pos2 = system.particles[*p2].pos;
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0, BLUE);
                }
                Constraint::Actuator { p1, p2, .. } => {
                    let pos1 = system.particles[*p1].pos;
                    let pos2 = system.particles[*p2].pos;
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0, PURPLE);
                }
                Constraint::Pin { p, pos } => {
                    let p_pos = system.particles[*p].pos;
                    draw_line(p_pos.x, p_pos.y, pos.x, pos.y, 2.0, GREEN);
                }
            }
        }

        // Render particles
        for p in &system.particles {
            draw_circle(p.pos.x, p.pos.y, 4.0, WHITE);
        }

        // --- 5. Render Flock ---
        for p in &positions {
            draw_circle(p.x as f32, p.y as f32, 3.0, RED);
        }

        next_frame().await;
    }
}
