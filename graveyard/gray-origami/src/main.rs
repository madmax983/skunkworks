use gray_scott::GrayScott;
use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::{Constraint, PbdSystem};

#[macroquad::main("Gray-Origami: Reaction-Diffusion Morphogenesis")]
async fn main() {
    let cols = 30;
    let rows = 30;

    // 1. Initialize Gray-Scott grid
    let mut gs = GrayScott::new(cols + 1, rows + 1);

    // Seed the chemical reaction in the center
    gs.add_chemical(cols / 2, rows / 2, 1.0);
    gs.add_chemical(cols / 2 + 1, rows / 2, 1.0);
    gs.add_chemical(cols / 2, rows / 2 + 1, 1.0);
    gs.add_chemical(cols / 2 - 1, rows / 2, 1.0);

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
        p_indices.push(system.add_particle(*pos, 1.0));
    }

    // Pin the four corners to keep the mesh from floating away
    let w = cols + 1;
    let top_left = 0;
    let top_right = cols;
    let bottom_left = rows * w;
    let bottom_right = rows * w + cols;

    system.add_pin_constraint(p_indices[top_left], points[top_left]);
    system.add_pin_constraint(p_indices[top_right], points[top_right]);
    system.add_pin_constraint(p_indices[bottom_left], points[bottom_left]);
    system.add_pin_constraint(p_indices[bottom_right], points[bottom_right]);

    // Store constraint mappings to know which Gray-Scott cell affects which constraint
    // (Constraint Index in system.constraints -> (x, y) grid coordinate)
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

                system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[right],
                    min_len,
                    max_len,
                    stiffness,
                );
                constraint_mapping.push((system.constraints.len() - 1, x, y));
            }

            // Vertical edge
            if y < rows {
                let down = (y + 1) * w + x;
                let dist = points[i].distance(points[down]);
                let min_len = dist * 0.2;
                let max_len = dist * 1.5;

                system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[down],
                    min_len,
                    max_len,
                    stiffness,
                );
                constraint_mapping.push((system.constraints.len() - 1, x, y));
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

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // Update Gray-Scott (multiple steps for visual speed)
        for _ in 0..10 {
            // Mitosis / Cell Division parameters
            gs.update(0.0367, 0.0649, 1.0);
        }

        // Map chemical V to physical constraints
        let v_layer = gs.v();
        for &(c_idx, x, y) in &constraint_mapping {
            let gs_idx = gs.get_index(x, y);
            let concentration = v_layer[gs_idx];

            // If V is high, factor goes towards 1.0 (expand/max_len)
            // If V is low, factor goes towards 0.0 (contract/min_len)
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
                let p00 = system.particles[p_indices[i]].pos;
                let p10 = system.particles[p_indices[i + 1]].pos;
                let p01 = system.particles[p_indices[i + w]].pos;

                let gs_idx = gs.get_index(x, y);
                let v = v_layer[gs_idx];

                // Color based on V concentration: deep purple to bright pink
                let r = 0.2 + v * 0.8;
                let g = 0.1;
                let b = 0.4 + v * 0.4;
                let color = Color::new(r, g, b, 1.0);

                // Draw wireframe outline instead of solid triangles (macroquad lacks draw_triangle_3d)
                draw_line_3d(p00, p10, color);
                draw_line_3d(p00, p01, color);
            }
        }

        // Draw last edges for wireframe
        for x in 0..cols {
            let i = rows * w + x;
            let p0 = system.particles[p_indices[i]].pos;
            let p1 = system.particles[p_indices[i + 1]].pos;
            draw_line_3d(p0, p1, Color::new(0.5, 0.5, 0.5, 1.0));
        }
        for y in 0..rows {
            let i = y * w + cols;
            let p0 = system.particles[p_indices[i]].pos;
            let p1 = system.particles[p_indices[i + w]].pos;
            draw_line_3d(p0, p1, Color::new(0.5, 0.5, 0.5, 1.0));
        }

        set_default_camera();

        // UI text
        draw_text(
            "Origami Morphogenesis (Gray-Scott)",
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            "V-Chemical -> Extends physical paper distance constraints",
            10.0,
            50.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
