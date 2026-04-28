use hyper_system::{math::Vec4, physics::PbdSystem4D};
use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};

#[macroquad::main("Hyper-Origami: Hyper-dimensional Morphogenesis")]
async fn main() {
    // 1. Initialize Origami Mesh parameters
    let params = MiuraParams {
        a: 10.0,
        b: 10.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };

    let grid_size = (10, 10);
    // Let's create an initial grid in 3D
    let initial_positions = generate_miura_grid(params, grid_size, 0.5);

    // 2. Initialize 4D Physics system
    let mut system = PbdSystem4D::new();
    let mut particle_indices = Vec::new();

    // Map the 3D origami vertices into the 4D physics system
    // We set their W coordinate to 0.0 initially.
    for pos in &initial_positions {
        let p = system
            .add_particle(Vec4::new(pos.x, pos.y, pos.z, 0.0), 1.0)
            .unwrap();
        particle_indices.push(p);
    }

    // Add structural distance constraints to maintain the mesh shape
    let width = grid_size.0 + 1;
    let height = grid_size.1 + 1;

    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            // Horizontal constraints
            if x < grid_size.0 {
                let right = i + 1;
                let dist = initial_positions[i].distance(initial_positions[right]);
                system.add_distance_constraint(particle_indices[i], particle_indices[right], dist);
            }
            // Vertical constraints
            if y < grid_size.1 {
                let down = i + width;
                let dist = initial_positions[i].distance(initial_positions[down]);
                system.add_distance_constraint(particle_indices[i], particle_indices[down], dist);
            }
            // Diagonal constraints for rigidity
            if x < grid_size.0 && y < grid_size.1 {
                let down_right = (y + 1) * width + (x + 1);
                let dist = initial_positions[i].distance(initial_positions[down_right]);
                system.add_distance_constraint(
                    particle_indices[i],
                    particle_indices[down_right],
                    dist,
                );
            }
        }
    }

    // Pin the center to keep it from flying away
    let center_x = width / 2;
    let center_y = height / 2;
    let center_i = center_y * width + center_x;
    let center_pos = initial_positions[center_i];
    system
        .add_pin_constraint(
            particle_indices[center_i],
            Vec4::new(center_pos.x, center_pos.y, center_pos.z, 0.0),
        )
        .unwrap();

    let camera = Camera3D {
        position: vec3(0.0, -100.0, 100.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let mut angle = 0.0;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        let dt = get_frame_time().min(0.032);
        angle += dt * 0.5;

        // Apply a hyper-dimensional rotation force
        // We push the vertices along the W axis, and rotate them in XW / YW planes
        for i in &particle_indices {
            let pos = system.particles[*i].pos;

            // Artificial force pushing into the W dimension based on distance from center
            let dist_from_center = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
            let w_force = (angle * 2.0 + dist_from_center * 0.1).sin() * 5.0;

            system.particles[*i].vel.w += w_force * dt;
            system.particles[*i].vel.y += (angle * 3.0 + pos.x * 0.1).cos() * 2.0 * dt;
            // Some 3D turbulence
        }

        // Step the 4D physics simulation
        system.step(dt, 5, 0.95);

        set_camera(&camera);

        // Render the mesh by projecting the 4D positions back to 3D
        let mut projected_positions = Vec::with_capacity(particle_indices.len());
        for i in &particle_indices {
            let p4 = system.particles[*i].pos;
            // Project down to 3D for rendering
            let p3 = p4.project_to_3d(50.0);
            projected_positions.push(vec3(p3.x, p3.y, p3.z));
        }

        // Draw lines
        for y in 0..height {
            for x in 0..width {
                let i = y * width + x;
                let p1 = projected_positions[i];

                if x < grid_size.0 {
                    let p2 = projected_positions[i + 1];
                    draw_line_3d(p1, p2, GREEN);
                }
                if y < grid_size.1 {
                    let p2 = projected_positions[i + width];
                    draw_line_3d(p1, p2, BLUE);
                }
            }
        }

        set_default_camera();

        draw_text("Hyper-Origami 4D Mesh", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 20.0, GRAY);

        next_frame().await
    }
}
