mod agent;
mod grid;
mod math;
mod monitor;

use agent::Agent;
use grid::SpinGrid4D;
use math::Vec4D;
use monitor::SystemMonitor;

use macroquad::prelude::*;

const AGENT_COUNT: usize = 64;
const GRID_SIZE: usize = 6; // 6^4 = 1296

// Convert angle (0-2PI) to Color (Hue)
fn angle_to_color(theta: f32) -> Color {
    let normalized = theta.rem_euclid(2.0 * std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
    hsl_to_rgb(normalized, 1.0, 0.5)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(r + m, g + m, b + m, 1.0)
}

#[macroquad::main("Chimera Glass")]
async fn main() {
    let mut grid = SpinGrid4D::new(GRID_SIZE);
    let mut monitor = SystemMonitor::new();
    let mut agents: Vec<Agent> = (0..AGENT_COUNT).map(|_| Agent::new_random()).collect();

    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 10.0f32;

    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    // Precompute vertex positions (Grid coords normalized to -1..1)
    let mut base_points = Vec::new();
    for i in 0..grid.hypersize {
        for j in 0..grid.depth {
            for k in 0..grid.height {
                for l in 0..grid.width {
                    let x = (l as f32 / (grid.width - 1) as f32) * 2.0 - 1.0;
                    let y = (k as f32 / (grid.height - 1) as f32) * 2.0 - 1.0;
                    let z = (j as f32 / (grid.depth - 1) as f32) * 2.0 - 1.0;
                    let w = (i as f32 / (grid.hypersize - 1) as f32) * 2.0 - 1.0;
                    base_points.push(Vec4D::new(x, y, z, w));
                }
            }
        }
    }

    loop {
        monitor.update();
        let dt = get_frame_time();

        // 1. Background Physics (Metropolis)
        // High CPU -> High Temp -> Random Flips
        let temp = monitor.cpu_usage * 2.0;
        let field = monitor.mem_usage * 2.0;

        // Only run background physics if temp is high enough to cause noise
        if temp > 0.1 {
            grid.step_metropolis(temp * 0.5, field);
        }

        // 2. Agent Update
        let mut new_agents = Vec::new();
        for agent in &mut agents {
            agent.update(&mut grid, &monitor);

            // Reproduction
            if agent.bio_energy > 200.0 {
                agent.bio_energy -= 100.0;
                new_agents.push(agent.clone_mutate());
            }
        }

        // Remove dead agents
        agents.retain(|a| a.bio_energy > 0.0);

        // Add newborns
        agents.append(&mut new_agents);

        // Repopulate if extinct
        if agents.len() < 4 {
            agents.push(Agent::new_random());
        }

        // 3. Visualization

        // Rotate Space
        let rot_speed = 0.1 + monitor.load_avg * 0.2;
        angle_xw += dt * rot_speed;
        angle_yw += dt * rot_speed * 0.5;
        angle_zw += dt * 0.02;

        // Camera Input
        if is_key_down(KeyCode::Left) {
            cam_angle_y += 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x += 2.0 * dt;
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x -= 2.0 * dt;
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 5.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 5.0 * dt;
        }

        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        let sx = 1.0 + monitor.cpu_usage * 1.5;
        let sy = 1.0 + monitor.mem_usage * 1.5;
        let sz = 1.0 + monitor.swap_usage * 1.5;
        let sw = 1.0 + monitor.load_avg;

        // Draw Spins
        for (idx, point) in base_points.iter().enumerate() {
            let theta = grid.spins[idx];

            let mut v = point.scale_dim(sx, sy, sz, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            let p3 = v.project_to_3d(4.0);

            let color = angle_to_color(theta);
            // Smaller size for spins to see agents better
            draw_sphere(p3, 0.04, None, color);
        }

        // Draw Agents
        for agent in &agents {
            // Map 0..1 pos to -1..1 range
            let p = Vec4D::new(
                agent.pos.x * 2.0 - 1.0,
                agent.pos.y * 2.0 - 1.0,
                agent.pos.z * 2.0 - 1.0,
                agent.pos.w * 2.0 - 1.0,
            );

            let mut v = p.scale_dim(sx, sy, sz, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            let p3 = v.project_to_3d(4.0);

            draw_sphere(p3, 0.15, None, agent.color);
        }

        // Draw Wireframe (Optional, maybe too cluttered with 1296 points)
        // Let's skip wireframe for clarity, the points define the shape.

        set_default_camera();

        draw_text("Chimera Glass", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("Agents: {}", agents.len()),
            10.0,
            50.0,
            20.0,
            GREEN,
        );
        draw_text(&format!("Temp (CPU): {:.2}", temp), 10.0, 70.0, 20.0, RED);
        draw_text(
            &format!("Field (RAM): {:.2}", field),
            10.0,
            90.0,
            20.0,
            BLUE,
        );

        next_frame().await
    }
}
