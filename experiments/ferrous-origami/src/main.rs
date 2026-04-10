use ferrous_core::Platter;
use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::{Constraint, PbdSystem};

// 🧬 Lineage Notes:
// Inherits the `origami` Miura-ori procedural mesh and `physics_pbd` from `gray-origami`.
// Inherits the `ferrous-core` Platter concept from `ferrous-fluid`.
// The phenotype expressed here creates a bidirectional feedback loop:
// The magnetic fluid pushes the mesh, while the tension in the mesh alters the fluid.

#[macroquad::main("Ferrous-Origami: Magnetic Morphogenesis")]
async fn main() {
    let cols = 30;
    let rows = 30;

    // 1. Initialize Ferrous Fluid Substrate
    let mut platter = Platter::new(cols + 1, rows + 1);

    // Seed the magnetic poles
    platter.accumulate(cols / 2, rows / 2, 5.0);
    platter.accumulate(cols / 2 + 1, rows / 2, 5.0);

    // 2. Initialize Origami Mesh and Physics
    let params = MiuraParams {
        a: 0.5,
        b: 0.5,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    let points = generate_miura_grid(params, (cols, rows), 0.5);
    let mut system = PbdSystem::new();
    let mut p_indices = Vec::with_capacity(points.len());

    for pos in &points {
        p_indices.push(system.add_particle(*pos, 1.0));
    }

    let w = cols + 1;
    let top_left = 0;
    let top_right = cols;
    let bottom_left = rows * w;
    let bottom_right = rows * w + cols;

    system.add_pin_constraint(p_indices[top_left], points[top_left]);
    system.add_pin_constraint(p_indices[top_right], points[top_right]);
    system.add_pin_constraint(p_indices[bottom_left], points[bottom_left]);
    system.add_pin_constraint(p_indices[bottom_right], points[bottom_right]);

    let mut constraint_mapping = Vec::new();
    let stiffness = 0.8;
    for y in 0..=rows {
        for x in 0..=cols {
            let i = y * w + x;
            if x < cols {
                let right = y * w + (x + 1);
                let dist = points[i].distance(points[right]);
                system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[right],
                    dist * 0.2,
                    dist * 1.5,
                    stiffness,
                );
                constraint_mapping.push((system.constraints.len() - 1, x, y));
            }
            if y < rows {
                let down = (y + 1) * w + x;
                let dist = points[i].distance(points[down]);
                system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[down],
                    dist * 0.2,
                    dist * 1.5,
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

        // Fluid Update
        platter.decay(0.99);

        // Interaction
        if is_mouse_button_down(MouseButton::Left) {
            platter.accumulate(cols / 2, rows / 2, 2.0);
        }

        // Map Magnetic field to physical constraints
        for &(c_idx, x, y) in &constraint_mapping {
            let magnetism = platter.get_magnetism(x, y) as f32;
            if let Constraint::Actuator { ref mut factor, .. } = system.constraints[c_idx] {
                let target_factor = (magnetism * 0.5).clamp(0.0, 1.0);
                *factor = *factor * 0.9 + target_factor * 0.1;
            }
        }

        system.step(0.016, 5);

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

                let magnetism = platter.get_magnetism(x, y) as f32;
                let color = Color::new(0.1 + magnetism, 0.2, 0.4 + magnetism * 0.5, 1.0);

                draw_line_3d(p00, p10, color);
                draw_line_3d(p00, p01, color);
            }
        }

        set_default_camera();

        draw_text(
            "Ferrous-Origami: Magnetic Morphogenesis",
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Magnetic Field -> Warps structural constraints",
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text("Click to inject magnetic flux", 10.0, 80.0, 20.0, RED);

        next_frame().await
    }
}
