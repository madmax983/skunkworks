use macroquad::prelude::*;
use chaos::{Attractor, LyapunovMonitor};
use monitor::SystemMonitor;

mod chaos;
mod monitor;

const TRAIL_LENGTH: usize = 10000;
const PHYSICS_STEPS_PER_FRAME: usize = 15;
const DT: f32 = 0.004;

#[macroquad::main("Lyapunov Sensor")]
async fn main() {
    let mut system_monitor = SystemMonitor::new();
    // Initial fetch
    system_monitor.update();
    let (sigma, rho, beta) = system_monitor.get_lorenz_params();

    // Start slightly off-center to ensure we fall into the attractor
    let mut attractor = Attractor::new(vec3(0.1, 0.0, 0.0), sigma, rho, beta);
    let mut monitor = LyapunovMonitor::new(attractor.pos, 1e-4);

    // Trail: (Position, Local Divergence)
    let mut trail: Vec<(Vec3, f32)> = Vec::with_capacity(TRAIL_LENGTH);

    // Camera
    let mut cam_angle: f32 = 0.0;
    let mut cam_dist: f32 = 80.0;
    let cam_height: f32 = 40.0;

    loop {
        // Update System
        system_monitor.update();
        let (sigma, rho, beta) = system_monitor.get_lorenz_params();

        // Smoothly update parameters? Or instant?
        attractor.sigma = sigma;
        attractor.rho = rho;
        attractor.beta = beta;

        // Physics
        for _ in 0..PHYSICS_STEPS_PER_FRAME {
            attractor.step(DT);
            monitor.step(&attractor, DT);

            trail.push((attractor.pos, monitor.current_divergence));
            if trail.len() > TRAIL_LENGTH {
                trail.remove(0);
            }
        }

        // Input
        if is_key_down(KeyCode::Left) { cam_angle -= 0.02; }
        if is_key_down(KeyCode::Right) { cam_angle += 0.02; }
        if is_key_down(KeyCode::Up) { cam_dist -= 1.0; }
        if is_key_down(KeyCode::Down) { cam_dist += 1.0; }
        cam_dist = cam_dist.clamp(10.0, 300.0);

        // Render
        clear_background(BLACK);

        let cam_pos = vec3(
            cam_dist * cam_angle.cos(),
            cam_height,
            cam_dist * cam_angle.sin(),
        );

        // Look at the center of the attractor roughly (0, 0, rho-ish)
        // For Lorenz, Z center is around rho-1.
        let target = vec3(0.0, 0.0, rho - 1.0);

        set_camera(&Camera3D {
            position: cam_pos,
            target,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_grid(20, 5.0, Color::new(0.2, 0.2, 0.2, 1.0), Color::new(0.1, 0.1, 0.1, 1.0));

        // Draw Trail
        // We draw segments.
        for i in 0..trail.len().saturating_sub(1) {
            let (p1, div1) = trail[i];
            let (p2, _) = trail[i+1];

            // Color based on divergence
            // Local divergence can be positive (expanding) or negative (contracting).
            // Usually between -5 and +5?
            // Let's map -1 to +1 to colors.
            // Blue = < 0 (Stable/Contracting)
            // Red = > 0 (Unstable/Expanding)
            // Green = 0 (Neutral)

            // Sigmoid-ish mapping
            let val = div1.tanh(); // -1 to 1

            // Simpler:
            // R = max(0, val)
            // B = max(0, -val)
            // G = 1 - abs(val)
            // Plus some base brightness
            let r = if val > 0.0 { val } else { 0.0 };
            let b = if val < 0.0 { -val } else { 0.0 };
            let g = 1.0 - val.abs();

            let color = Color::new(r, g, b, 1.0);

            draw_line_3d(p1, p2, color);
        }

        // Draw Head
        draw_sphere(attractor.pos, 0.8, None, WHITE);

        // Draw Shadow Head (debug)
        // draw_sphere(monitor.shadow_pos, 0.2, YELLOW);

        set_default_camera();

        // HUD
        draw_text("Lyapunov Sensor", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 40.0, 20.0, GRAY);

        let avg_lle = monitor.get_lle(DT);
        draw_text(&format!("LLE (Global): {:.4}", avg_lle), 10.0, 70.0, 20.0, GOLD);
        draw_text(&format!("LLE (Local):  {:.4}", monitor.current_divergence), 10.0, 90.0, 20.0, GOLD);

        draw_text(&format!("CPU: {:.1}% -> Rho: {:.1}", system_monitor.cpu_usage, rho), 10.0, 120.0, 20.0, GREEN);
        draw_text(&format!("MEM: {:.1}% -> Beta: {:.2}", system_monitor.mem_usage, beta), 10.0, 140.0, 20.0, BLUE);

        // Stability Status
        let status = if avg_lle > 0.1 { "CHAOS" } else { "STABLE" };
        let status_color = if avg_lle > 0.1 { RED } else { BLUE };
        let status_text_width = measure_text(status, None, 40, 1.0).width;
        draw_text(status, screen_width() - status_text_width - 20.0, 50.0, 40.0, status_color);

        // Help
        draw_text("Arrows: Camera", 10.0, screen_height() - 20.0, 16.0, GRAY);

        next_frame().await
    }
}
