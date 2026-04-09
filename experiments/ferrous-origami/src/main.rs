use ferrous_core::Platter;
use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::{Constraint, PbdSystem};

#[macroquad::main("Ferrous-Origami: Magnetic Morphogenesis")]
async fn main() {
    let cols = 30;
    let rows = 30;

    // 1. Initialize Magnetic Platter
    let mut platter = Platter::new(cols + 1, rows + 1);

    // Seed the magnetic field in the center
    platter.accumulate(cols / 2, rows / 2, 1.0);
    platter.accumulate(cols / 2 + 1, rows / 2, 1.0);
    platter.accumulate(cols / 2, rows / 2 + 1, 1.0);
    platter.accumulate(cols / 2 - 1, rows / 2, 1.0);

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

    // Create horizontal distance constraints
    for j in 0..=rows {
        for i in 0..cols {
            let idx1 = j * w + i;
            let idx2 = j * w + i + 1;
            let d = points[idx1].distance(points[idx2]);
            system.add_distance_constraint(p_indices[idx1], p_indices[idx2], d);
        }
    }

    // Create vertical distance constraints
    for j in 0..rows {
        for i in 0..=cols {
            let idx1 = j * w + i;
            let idx2 = (j + 1) * w + i;
            let d = points[idx1].distance(points[idx2]);
            system.add_distance_constraint(p_indices[idx1], p_indices[idx2], d);
        }
    }

    // Diagonal constraints for stability
    for j in 0..rows {
        for i in 0..cols {
            let idx1 = j * w + i;
            let idx2 = (j + 1) * w + i + 1;
            let d = points[idx1].distance(points[idx2]);
            system.add_distance_constraint(p_indices[idx1], p_indices[idx2], d);
        }
    }

    // Save original rest lengths to modulate them
    let mut original_lengths = Vec::new();
    for c in &system.constraints {
        if let Constraint::Distance { rest_length, .. } = c {
            original_lengths.push(*rest_length);
        } else {
            original_lengths.push(0.0);
        }
    }

    // Camera state
    let mut cam_yaw: f32 = std::f32::consts::PI / 4.0;
    let mut cam_pitch: f32 = std::f32::consts::PI / 4.0;
    let mut cam_dist: f32 = 40.0;
    let mut last_mouse_pos = mouse_position();

    loop {
        // Input for Camera
        let mouse_pos = mouse_position();
        let delta = vec2(
            mouse_pos.0 - last_mouse_pos.0,
            mouse_pos.1 - last_mouse_pos.1,
        );
        last_mouse_pos = mouse_pos;

        if is_mouse_button_down(MouseButton::Left) {
            cam_yaw -= delta.x * 0.01;
            cam_pitch += delta.y * 0.01;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }
        cam_dist -= mouse_wheel().1 * 0.5;

        // Spread magnetic field (simple blur/diffusion)
        let mut new_magnetism = vec![0.0; platter.magnetism().len()];
        let w_plat = platter.width();
        let h_plat = platter.height();
        for y in 1..(h_plat - 1) {
            for x in 1..(w_plat - 1) {
                let current = platter.get(x, y);
                let left = platter.get(x - 1, y);
                let right = platter.get(x + 1, y);
                let up = platter.get(x, y - 1);
                let down = platter.get(x, y + 1);

                let laplacian = left + right + up + down - 4.0 * current;
                let next_val = current + (0.2 * laplacian); // diffusion rate 0.2
                new_magnetism[y * w_plat + x] = next_val;
            }
        }
        for y in 1..(h_plat - 1) {
            for x in 1..(w_plat - 1) {
                platter.accumulate(x, y, new_magnetism[y * w_plat + x] - platter.get(x, y));
            }
        }

        // Pulse random magnetic spots
        if rand::gen_range(0, 10) == 0 {
            let rx = rand::gen_range(1, cols - 1);
            let ry = rand::gen_range(1, rows - 1);
            platter.accumulate(rx, ry, 2.0);
        }

        // Apply Magnetic Morphogenesis!
        // We modify the rest_lengths of the constraints based on the magnetic field.
        let mut c_idx = 4;

        // Horizontal constraints
        for j in 0..=rows {
            for i in 0..cols {
                let mag = platter.get(i.min(cols-1), j.min(rows-1)) as f32;
                let multiplier = 0.5 + mag.clamp(0.0, 1.0) * 1.0;

                if let Constraint::Distance { ref mut rest_length, .. } = system.constraints[c_idx] {
                    *rest_length = original_lengths[c_idx] * multiplier;
                }
                c_idx += 1;
            }
        }

        // Vertical constraints
        for j in 0..rows {
            for i in 0..=cols {
                let mag = platter.get(i.min(cols-1), j.min(rows-1)) as f32;
                let multiplier = 0.5 + mag.clamp(0.0, 1.0) * 1.0;

                if let Constraint::Distance { ref mut rest_length, .. } = system.constraints[c_idx] {
                    *rest_length = original_lengths[c_idx] * multiplier;
                }
                c_idx += 1;
            }
        }

        // Diagonal constraints
        for j in 0..rows {
            for i in 0..cols {
                let mag = platter.get(i.min(cols-1), j.min(rows-1)) as f32;
                let multiplier = 0.5 + mag.clamp(0.0, 1.0) * 1.0;

                if let Constraint::Distance { ref mut rest_length, .. } = system.constraints[c_idx] {
                    *rest_length = original_lengths[c_idx] * multiplier;
                }
                c_idx += 1;
            }
        }

        // Gravity & Physics Step
        for p in &mut system.particles {
            if p.inv_mass > 0.0 {
                p.vel.y -= 2.0 * 0.016; // Slight gravity
            }
        }
        system.step(0.016, 5);

        // Rendering
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        let cam_pos = vec3(
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
        );

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        // Draw the mesh using lines
        for c in &system.constraints {
            if let Constraint::Distance { p1, p2, .. } = c {
                let pos1 = system.particles[*p1].pos;
                let pos2 = system.particles[*p2].pos;

                // Color based on tension? Or just static.
                // Let's color based on the magnetic field at that position approximately
                let _idx1 = *p1;
                // mapping back to i,j is tricky here, so let's just use a base color
                let color = Color::new(0.6, 0.6, 0.8, 0.8);

                draw_line_3d(
                    vec3(pos1.x, pos1.y, pos1.z),
                    vec3(pos2.x, pos2.y, pos2.z),
                    color,
                );
            }
        }

        // Draw magnetic substrate as a faint background grid underneath
        for j in 0..rows {
            for i in 0..cols {
                let mag = platter.get(i, j) as f32;
                if mag > 0.1 {
                    let cx = (i as f32 - cols as f32 / 2.0) * params.a;
                    let cy = (j as f32 - rows as f32 / 2.0) * params.b;

                    let size = mag.clamp(0.1, 1.0) * 0.4;
                    draw_cube(vec3(cx, -5.0, cy), vec3(size, 0.1, size), None, Color::new(mag, 0.0, 0.0, 0.5));
                }
            }
        }

        set_default_camera();
        draw_text("Ferrous-Origami: Magnetic Morphogenesis", 10.0, 20.0, 30.0, WHITE);
        draw_text("Drag to rotate, Scroll to zoom", 10.0, 50.0, 20.0, GRAY);

        // decay magnetic field slightly
        platter.decay(0.99);

        next_frame().await
    }
}
