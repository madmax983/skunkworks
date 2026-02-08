use macroquad::prelude::*;
use system_attractor::simulation::{update_particles, Particle, SystemMonitor};

#[macroquad::main("System Attractor")]
async fn main() {
    let mut monitor = SystemMonitor::new();

    // Target 50,000 particles
    // Render budget might be tight for lines in macroquad immediate mode
    let num_particles = 50_000;
    let mut particles: Vec<Particle> = Vec::with_capacity(num_particles);

    // Initialize particles near origin with some spread
    for _ in 0..num_particles {
        let x = rand::gen_range(-10.0, 10.0);
        let y = rand::gen_range(-10.0, 10.0);
        let z = rand::gen_range(10.0, 40.0);
        particles.push(Particle::new(x, y, z));
    }

    // Camera State
    let mut cam_dist = 80.0;
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.0;

    // Simulation timing
    let dt = 0.005; // Smaller step for stability

    loop {
        // Update Monitor (every frame for responsiveness, sysinfo caches anyway or is fast enough)
        monitor.update();

        // Update Physics
        // Run physics multiple times per frame to speed up simulation relative to real time?
        // Or just once. Let's do 2 steps per frame for speed.
        for _ in 0..2 {
            update_particles(&mut particles, &monitor.params, dt);
        }

        // Camera Controls
        if is_key_down(KeyCode::Left) {
            cam_yaw -= 0.02;
        }
        if is_key_down(KeyCode::Right) {
            cam_yaw += 0.02;
        }
        if is_key_down(KeyCode::Up) {
            cam_pitch = (cam_pitch + 0.02).clamp(-1.5, 1.5);
        }
        if is_key_down(KeyCode::Down) {
            cam_pitch = (cam_pitch - 0.02).clamp(-1.5, 1.5);
        }
        // Zoom
        if is_key_down(KeyCode::W) {
            cam_dist -= 0.5;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 0.5;
        }

        // Reset
        if is_key_pressed(KeyCode::R) {
            particles.iter_mut().for_each(|p| {
                p.pos = vec3(
                    rand::gen_range(-10.0, 10.0),
                    rand::gen_range(-10.0, 10.0),
                    rand::gen_range(10.0, 40.0),
                );
                p.vel = vec3(0., 0., 0.);
            });
        }

        clear_background(BLACK);

        // Setup Camera
        let cam_pos = vec3(
            cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );
        let target = vec3(0.0, 0.0, 25.0); // Look at center (z~25)

        set_camera(&Camera3D {
            position: cam_pos,
            target,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Draw Particles
        // Drawing points as small lines
        // Rendering only a subset if FPS drops?
        // Let's render all 50k and see. If it's slow, user can reduce count in code.
        // I'll render them as short lines based on velocity direction for "flow" look.
        // Actually, just points (tiny lines) is faster.
        for p in &particles {
            // Simple point
            // draw_line_3d is creating geometry every call.
            // Ideally we use a mesh, but macroquad doesn't expose point primitive easily in high level.
            // We can use `draw_line_3d` with length 0.05.
            draw_line_3d(p.pos, p.pos + vec3(0.05, 0.05, 0.05), p.color);
        }

        // Draw Axis
        draw_line_3d(vec3(0., 0., 0.), vec3(10., 0., 0.), RED);
        draw_line_3d(vec3(0., 0., 0.), vec3(0., 10., 0.), GREEN);
        draw_line_3d(vec3(0., 0., 0.), vec3(0., 0., 10.), BLUE);

        set_default_camera();

        // HUD
        draw_text("System Attractor", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 60.0, 20.0, LIGHTGRAY);
        draw_text(
            &format!("Particles: {}", num_particles),
            10.0,
            80.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Sigma (CPU): {:.2}", monitor.params.sigma),
            10.0,
            110.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("Rho (RAM): {:.2}", monitor.params.rho),
            10.0,
            130.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("Beta: {:.2}", monitor.params.beta),
            10.0,
            150.0,
            20.0,
            GREEN,
        );
        draw_text(
            "Controls: Arrows to rotate, W/S zoom, R reset",
            10.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
