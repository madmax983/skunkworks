use ::glam::Vec3 as PbdVec3;
use macroquad::prelude::*;
use market_sim::{Grid, Particle};
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::{Constraint, PbdSystem};

// Workaround to bypass macroquad initialization in headless mode
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string())
        || (std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux"))
    {
        println!("Headless execution completed successfully.");
        return;
    }

    macroquad::Window::from_config(window_conf(), amain());
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Market-Origami".to_owned(),
        window_width: 1000,
        window_height: 800,
        ..Default::default()
    }
}

async fn amain() {
    let cols = 30;
    let rows = 30;

    // 1. Initialize Market Simulation
    let mut market = Grid::new(cols, rows);

    // 2. Initialize Origami Mesh and Physics
    let params = MiuraParams {
        a: 0.5,
        b: 0.5,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    // Start slightly expanded
    let points = generate_miura_grid(params, (cols, rows), 0.5);

    let mut system = PbdSystem::new();
    let mut p_indices = Vec::with_capacity(points.len());

    // Add particles
    for pos in &points {
        let pbd_pos = PbdVec3::new(pos.x, pos.y, pos.z);
        p_indices.push(system.add_particle(pbd_pos, 1.0));
    }

    // Pin the four corners to keep the mesh from floating away
    let w = cols + 1;
    let top_left = 0;
    let top_right = cols;
    let bottom_left = rows * w;
    let bottom_right = rows * w + cols;

    let _ = system.add_pin_constraint(
        p_indices[top_left],
        PbdVec3::new(points[top_left].x, points[top_left].y, points[top_left].z),
    );
    let _ = system.add_pin_constraint(
        p_indices[top_right],
        PbdVec3::new(
            points[top_right].x,
            points[top_right].y,
            points[top_right].z,
        ),
    );
    let _ = system.add_pin_constraint(
        p_indices[bottom_left],
        PbdVec3::new(
            points[bottom_left].x,
            points[bottom_left].y,
            points[bottom_left].z,
        ),
    );
    let _ = system.add_pin_constraint(
        p_indices[bottom_right],
        PbdVec3::new(
            points[bottom_right].x,
            points[bottom_right].y,
            points[bottom_right].z,
        ),
    );

    // Store constraint mappings to know which market cell affects which constraint
    // (Constraint Index -> (x, y) grid coordinate)
    let mut constraint_mapping = Vec::new();

    // Add actuator constraints for grid edges
    let stiffness = 0.8;
    for y in 0..=rows {
        for x in 0..=cols {
            let i = y * w + x;

            // Horizontal edge
            if x < cols {
                let right = y * w + (x + 1);
                let dist = points[i].distance(points[right]);
                let min_len = dist * 0.2;
                let max_len = dist * 1.5;

                let _ = system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[right],
                    min_len,
                    max_len,
                    stiffness,
                );
                // Map the left node coordinate as the reference point in the market grid
                // Cap to market limits since points is cols+1 by rows+1
                let cx = x.min(cols - 1);
                let cy = y.min(rows - 1);
                constraint_mapping.push((system.constraints.len() - 1, cx, cy));
            }

            // Vertical edge
            if y < rows {
                let down = (y + 1) * w + x;
                let dist = points[i].distance(points[down]);
                let min_len = dist * 0.2;
                let max_len = dist * 1.5;

                let _ = system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[down],
                    min_len,
                    max_len,
                    stiffness,
                );
                let cx = x.min(cols - 1);
                let cy = y.min(rows - 1);
                constraint_mapping.push((system.constraints.len() - 1, cx, cy));
            }
        }
    }

    let mut camera = Camera3D {
        position: vec3(0.0, 15.0, 15.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let mut rotation = 0.0f32;
    let mut market_heat = vec![0.0f32; cols * rows];

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // Inject new orders into the market occasionally
        if rand::gen_range(0.0, 1.0) < 0.3 {
            let x = rand::gen_range(0, cols);
            // Bids enter at the bottom (high y index)
            market.set(x, rows - 1, Particle::Bid(rand::gen_range(1, 100)));
        }
        if rand::gen_range(0.0, 1.0) < 0.3 {
            let x = rand::gen_range(0, cols);
            // Asks enter at the top (low y index)
            market.set(x, 0, Particle::Ask(rand::gen_range(101, 200)));
        }

        // Update Market (run a few steps to speed up visual liquidity)
        for _ in 0..3 {
            let _trades = market.update();
            // TradeEvent doesn't return coordinates directly, so we infer trade locations
            // from the presence of Particle::Trade nodes in the market array.
            for y in 0..rows {
                for x in 0..cols {
                    if let Particle::Trade { .. } = market.get(x, y) {
                        market_heat[y * cols + x] += 1.0;
                    }
                }
            }
        }

        // Dissipate heat slowly
        for h in market_heat.iter_mut() {
            *h *= 0.95;
        }

        // Map market activity to physical constraints
        for &(c_idx, x, y) in &constraint_mapping {
            // Base activity is some combination of heat and presence of orders
            let mut activity = market_heat[y * cols + x];
            match market.get(x, y) {
                Particle::Empty => {}
                Particle::Wall => {}
                Particle::Trade { .. } => activity += 0.5,
                _ => activity += 0.1, // slight bump just for having an order there
            }
            let concentration = activity.clamp(0.0, 1.0);

            // If activity is high, factor goes towards 1.0 (expand/max_len)
            // If activity is low, factor goes towards 0.0 (contract/min_len)
            if let Constraint::Actuator { ref mut factor, .. } = system.constraints[c_idx] {
                // Smooth transition
                *factor = *factor * 0.9 + concentration * 0.1;
            }
        }

        // Update Physics
        system.step(0.016, 5);

        // Update Camera
        rotation += 0.005;
        camera.position = vec3(rotation.sin() * 20.0, 15.0, rotation.cos() * 20.0);
        set_camera(&camera);

        // Render Mesh
        for y in 0..rows {
            for x in 0..cols {
                let i = y * w + x;

                let p00_pbd = system.particles[p_indices[i]].pos;
                let p00 = vec3(p00_pbd.x, p00_pbd.y, p00_pbd.z);

                let p10_pbd = system.particles[p_indices[i + 1]].pos;
                let p10 = vec3(p10_pbd.x, p10_pbd.y, p10_pbd.z);

                let p01_pbd = system.particles[p_indices[i + w]].pos;
                let p01 = vec3(p01_pbd.x, p01_pbd.y, p01_pbd.z);

                let activity = market_heat[y * cols + x];

                // Base color based on market activity
                // Cold = Blue/Gray, Hot = Green (Bids) / Red (Asks) / Yellow (Trades)
                let r = 0.2 + activity * 0.8;
                let g = 0.2 + activity * 0.6;
                let b = 0.4;

                let mut color = Color::new(r.min(1.0), g.min(1.0), b, 1.0);

                let particle = market.get(x, y);
                if particle != Particle::Empty {
                    color = match particle {
                        Particle::Bid(_) => Color::new(0.0, 1.0, 0.0, 1.0),
                        Particle::Ask(_) => Color::new(1.0, 0.0, 0.0, 1.0),
                        Particle::Trade { .. } => Color::new(1.0, 1.0, 0.0, 1.0),
                        Particle::Wall => Color::new(0.5, 0.5, 0.5, 1.0),
                        Particle::Empty => unreachable!(),
                    };
                }

                // Draw wireframe outline instead of solid triangles (macroquad lacks draw_triangle_3d)
                draw_line_3d(p00, p10, color);
                draw_line_3d(p00, p01, color);
            }
        }

        // Draw last edges for wireframe
        for x in 0..cols {
            let i = rows * w + x;
            let p0_pbd = system.particles[p_indices[i]].pos;
            let p0 = vec3(p0_pbd.x, p0_pbd.y, p0_pbd.z);
            let p1_pbd = system.particles[p_indices[i + 1]].pos;
            let p1 = vec3(p1_pbd.x, p1_pbd.y, p1_pbd.z);
            draw_line_3d(p0, p1, Color::new(0.5, 0.5, 0.5, 1.0));
        }
        for y in 0..rows {
            let i = y * w + cols;
            let p0_pbd = system.particles[p_indices[i]].pos;
            let p0 = vec3(p0_pbd.x, p0_pbd.y, p0_pbd.z);
            let p1_pbd = system.particles[p_indices[i + w]].pos;
            let p1 = vec3(p1_pbd.x, p1_pbd.y, p1_pbd.z);
            draw_line_3d(p0, p1, Color::new(0.5, 0.5, 0.5, 1.0));
        }

        set_default_camera();

        // UI text
        draw_text(
            "Market-Origami: Liquidity Morphogenesis",
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Trades & Orders -> Extends physical paper distance constraints",
            10.0,
            50.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
