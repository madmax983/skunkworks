use macroquad::prelude::*;
use macroquad::color::hsl_to_rgb;
mod physics;
use physics::{Body, integrate, G};

fn window_conf() -> Conf {
    Conf {
        window_title: "Genesis: Tidal Channel".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut bodies: Vec<Body> = Vec::new();
    let mut singularity_pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut singularity_mass = 5000.0;

    // Create a circle texture for ellipses
    let texture_size = 64;
    let mut image = Image::gen_image_color(texture_size, texture_size, Color::new(0.0, 0.0, 0.0, 0.0));
    for y in 0..texture_size {
        for x in 0..texture_size {
            let dx = x as f32 - texture_size as f32 / 2.0;
            let dy = y as f32 - texture_size as f32 / 2.0;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < texture_size as f32 / 2.0 {
                // Soft edge
                let alpha = ((texture_size as f32 / 2.0 - dist) / (texture_size as f32 / 8.0)).min(1.0);
                image.set_pixel(x as u32, y as u32, Color::new(1.0, 1.0, 1.0, alpha));
            }
        }
    }
    let circle_texture = Texture2D::from_image(&image);

    let mut fracture_threshold: f32 = 20.0; // Adjustable
    let min_radius = 2.0;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time().min(0.05); // Cap dt

        // Input
        if is_mouse_button_down(MouseButton::Right) {
            singularity_pos = mouse_position().into();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos: Vec2 = mouse_position().into();
            // Spawn with velocity towards singularity + tangent
            let to_center = singularity_pos - mpos;
            let dist = to_center.length();
            let dir = to_center / dist;
            let tangent = vec2(-dir.y, dir.x);

            // Initial orbital velocity for circular orbit is sqrt(GM/r)
            let orbital_vel = (G * singularity_mass / dist).sqrt();

            // Add some randomness
            let vel = tangent * orbital_vel * rand::gen_range(0.8, 1.2) + dir * rand::gen_range(-10.0, 10.0);

            bodies.push(Body {
                pos: mpos,
                vel,
                mass: rand::gen_range(10.0, 50.0),
                radius: rand::gen_range(8.0, 15.0),
                color: hsl_to_rgb(rand::gen_range(0.0, 1.0), 0.8, 0.6),
                _id: rand::rand() as u64,
            });
        }

        // Scroll to adjust singularity mass
        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            singularity_mass += wheel_y * 500.0;
            singularity_mass = singularity_mass.max(100.0);
        }

        if is_key_down(KeyCode::Up) {
            fracture_threshold += 0.1;
        }
        if is_key_down(KeyCode::Down) {
            fracture_threshold -= 0.1;
            fracture_threshold = fracture_threshold.max(1.0);
        }

        // Physics Step
        let substeps = 4;
        let sdt = dt / substeps as f32;

        for _ in 0..substeps {
            // Integration
            integrate(&mut bodies, sdt, singularity_pos, singularity_mass);

            // Fracture Check
            let mut new_bodies = Vec::new();
            let mut keep_indices = Vec::new();

            for (i, body) in bodies.iter().enumerate() {
                let tidal_force = body.get_tidal_force_magnitude(singularity_pos, singularity_mass);

                // Visualization color shift based on stress
                // We can't mutate color in this immutable iterator easily if we want to also fracture
                // So we'll update color in a separate pass or just during draw.

                // Break condition: force > threshold AND radius > min
                if tidal_force > fracture_threshold && body.radius > min_radius {
                    let (b1, b2) = body.fracture();
                    new_bodies.push(b1);
                    new_bodies.push(b2);
                    // Don't keep original
                } else {
                    keep_indices.push(i);
                }
            }

            if !new_bodies.is_empty() {
                // Rebuild bodies vector efficiently
                // Simple approach: create new vec
                let mut next_bodies = Vec::with_capacity(bodies.len() + new_bodies.len());
                for &i in &keep_indices {
                    next_bodies.push(bodies[i].clone());
                }
                next_bodies.append(&mut new_bodies);
                bodies = next_bodies;
            }

            // Consume Check (Singularity Horizon)
            bodies.retain(|b| {
                let dist_sq = (b.pos - singularity_pos).length_squared();
                dist_sq > 100.0 // Singularity radius squared (10^2)
            });
        }

        // Draw

        // Draw Singularity
        draw_circle(singularity_pos.x, singularity_pos.y, 10.0, WHITE);
        draw_circle_lines(singularity_pos.x, singularity_pos.y, 12.0, 2.0, Color::new(0.0, 1.0, 1.0, 1.0));

        // Draw Roche Limit Visual (Approximate for radius=10 body)
        // F = 2GMR/r^3 = Threshold -> r^3 = 2GMR/Threshold -> r = cbrt(...)
        let approx_roche_radius = ((2.0 * G * singularity_mass * 10.0) / fracture_threshold).cbrt();
        draw_circle_lines(singularity_pos.x, singularity_pos.y, approx_roche_radius, 1.0, Color::new(1.0, 0.0, 0.0, 0.3));


        // Draw Bodies
        for body in &bodies {
            let tidal_force = body.get_tidal_force_magnitude(singularity_pos, singularity_mass);
            let stress = (tidal_force / fracture_threshold).min(1.0);

            // Color shift from Base -> Red based on stress
            let draw_color = Color::new(
                body.color.r * (1.0 - stress) + 1.0 * stress,
                body.color.g * (1.0 - stress),
                body.color.b * (1.0 - stress),
                body.color.a
            );

            // Stretch Ellipse along radius
            let to_center = singularity_pos - body.pos;
            let rotation = to_center.y.atan2(to_center.x);

            // Stretch factor: 1.0 + stress * 2.0
            let stretch_x = 1.0 + stress * 2.0;
            let stretch_y = 1.0 / stretch_x; // Conserve area roughly visual

            draw_texture_ex(
                &circle_texture,
                body.pos.x - body.radius * stretch_x,
                body.pos.y - body.radius * stretch_y,
                draw_color,
                DrawTextureParams {
                    dest_size: Some(vec2(body.radius * 2.0 * stretch_x, body.radius * 2.0 * stretch_y)),
                    rotation,
                    ..Default::default()
                }
            );
        }

        // UI
        draw_text("Genesis: Tidal Channel", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Bodies: {}", bodies.len()), 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Mass: {:.0} (Scroll)", singularity_mass), 20.0, 80.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Fracture Threshold: {:.1} (Up/Down)", fracture_threshold), 20.0, 100.0, 20.0, LIGHTGRAY);
        draw_text("Left Click: Spawn | Right Click: Move Singularity", 20.0, screen_height() - 20.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
