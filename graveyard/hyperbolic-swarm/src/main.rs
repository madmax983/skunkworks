use crate::swarm::Swarm;
use macroquad::prelude::*;

pub mod boid;
pub mod swarm;

#[macroquad::main("Hyperbolic Swarm")]
async fn main() {
    let mut swarm = Swarm::new(50);

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Dark Blue

        // Set up camera to view the unit disk
        let aspect = screen_width() / screen_height();
        let scale = 1.0 / 1.1; // Slightly smaller than screen
        let zoom = if aspect > 1.0 {
            vec2(scale / aspect, scale)
        } else {
            vec2(scale, scale * aspect)
        };

        set_camera(&Camera2D {
            zoom,
            target: vec2(0.0, 0.0),
            ..Default::default()
        });

        // Draw Boundary
        draw_circle_lines(0.0, 0.0, 1.0, 0.005, WHITE);

        // Update Swarm
        swarm.update();

        // Draw Boids
        for boid in &swarm.boids {
            let x = boid.pos.re as f32;
            let y = boid.pos.im as f32;

            // Draw as triangle pointing in velocity direction
            // Velocity in Poincaré disk is a vector in the tangent space?
            // Or just displacement.
            // Angle of velocity vector gives screen-space heading roughly.
            let angle = boid.vel.im.atan2(boid.vel.re) as f32;

            let size = 0.02;
            let p1 = vec2(x + angle.cos() * size, y + angle.sin() * size);
            let p2 = vec2(
                x + (angle + 2.5).cos() * size * 0.7,
                y + (angle + 2.5).sin() * size * 0.7,
            );
            let p3 = vec2(
                x + (angle - 2.5).cos() * size * 0.7,
                y + (angle - 2.5).sin() * size * 0.7,
            );

            draw_triangle(p1, p2, p3, boid.color);
        }

        set_default_camera();
        draw_text("Hyperbolic Swarm", 10.0, 30.0, 30.0, WHITE);
        draw_text("Flocking in Poincaré Disk", 10.0, 50.0, 20.0, GRAY);
        draw_text(
            &format!("Boids: {}", swarm.boids.len()),
            10.0,
            70.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
