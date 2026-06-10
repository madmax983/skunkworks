use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::{Constraint, PbdSystem};
use resonance_audio::physics::PhysicsGrid;

// 🧬 Lineage Notes:
// Inherits the `origami` Miura-ori procedural mesh and `physics_pbd` from `gray-origami`.
// Inherits the `resonance-audio` acoustic wave tank from `gray-resonance` / `resonance-audio`.
// The phenotype expressed here creates an acoustic-morphogenetic visual loop:
// The continuous 2D acoustic pressure acts as an exciter to dynamically actuate
// the 3D distance constraints of the soft-body mesh.

#[macroquad::main("Resonance-Origami: Acoustic Morphogenesis")]
async fn main() {
    let cols = 30;
    let rows = 30;

    // 1. Initialize Acoustic Resonance Substrate
    let mut grid = PhysicsGrid::new(cols + 1, rows + 1);

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

    // Track which actuator constraint is mapped to which grid cell
    let mut constraint_mapping = Vec::new();
    let stiffness = 0.8;

    for y in 0..=rows {
        for x in 0..=cols {
            let i = y * w + x;

            // Horizontal edge
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

            // Vertical edge
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

            // Diagonal
            if x < cols && y < rows {
                let diag = (y + 1) * w + (x + 1);
                let dist = points[i].distance(points[diag]);
                system.add_actuator_constraint(
                    p_indices[i],
                    p_indices[diag],
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

        // Interaction
        if is_mouse_button_down(MouseButton::Left) {
            // Drop an acoustic pluck in the center of the grid
            grid.pluck(cols / 2, rows / 2, 5.0);
        }

        // Acoustic Update
        grid.step();

        // Map Acoustic Pressure to physical constraints
        for &(c_idx, x, y) in &constraint_mapping {
            // Acoustic pressure usually oscillates around 0. We take absolute value
            // or just let it expand/contract based on wave phase.
            let pressure = grid.get(x, y);

            if let Constraint::Actuator { ref mut factor, .. } = system.constraints[c_idx] {
                // Map pressure to a factor between 0.0 and 1.0
                // Normalizing roughly against expected wave amplitude
                let target_factor = (0.5 + pressure * 0.2).clamp(0.0_f32, 1.0_f32);
                // Smooth the factor to avoid popping geometry
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
                let right = y * w + (x + 1);
                let down = (y + 1) * w + x;

                let p1 = system.particles[p_indices[i]].pos;

                let mut pressure_color = grid.get(x, y).abs() * 2.0;
                pressure_color = pressure_color.clamp(0.0_f32, 1.0_f32);

                // Color maps pressure: high pressure is cyan, low is dark blue
                let col = Color::new(
                    0.1 + pressure_color * 0.2,
                    0.2 + pressure_color * 0.6,
                    0.4 + pressure_color * 0.6,
                    1.0,
                );

                if x < cols {
                    let p2 = system.particles[p_indices[right]].pos;
                    draw_line_3d(p1, p2, col);
                }
                if y < rows {
                    let p2 = system.particles[p_indices[down]].pos;
                    draw_line_3d(p1, p2, col);
                }
            }
        }

        set_default_camera();
        draw_text("Resonance-Origami", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Click to pluck acoustic wave tank",
            10.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}
