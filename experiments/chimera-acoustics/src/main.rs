mod agent;
mod audio;
mod grid;
mod math;
mod monitor;

use agent::Agent;
use audio::init_audio;
use grid::{AcousticGrid4D, Point4D, GRID_SIZE};
use math::Vec4;
use monitor::SystemMonitor;

use macroquad::prelude::*;
use crossbeam_channel::bounded;

const AGENT_COUNT: usize = 32;

#[macroquad::main("Chimera Acoustics")]
async fn main() {
    // Audio Setup
    let (tx, rx) = bounded(4096);
    let _stream = match init_audio(rx) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Audio init failed: {}", e);
            return;
        }
    };

    let mut grid = AcousticGrid4D::new();
    let mut monitor = SystemMonitor::new();
    let mut agents: Vec<Agent> = (0..AGENT_COUNT).map(|i| Agent::new_random(i as u64)).collect();

    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 10.0f32;

    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    // Precompute visual grid points
    let mut visual_grid_points = Vec::new();
    for w in 0..GRID_SIZE {
        for z in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    // Map 0..SIZE to -1.0..1.0
                    let fx = (x as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
                    let fy = (y as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
                    let fz = (z as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
                    let fw = (w as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
                    visual_grid_points.push(Vec4::new(fx, fy, fz, fw));
                }
            }
        }
    }

    let mut step_counter: u64 = 0;

    loop {
        monitor.update();
        let dt = get_frame_time();

        // 1. Physics & Audio Loop
        // Generate enough samples to keep buffer healthy (aim for 2048)
        // Main thread will block on send if full, or loop until full.
        // We use `is_full()` check implicitly via loop condition `len() < 2048`.

        let c2 = 0.1 + monitor.cpu_usage * 0.4;
        let base_damping = 0.99 - monitor.mem_usage * 0.05;

        // Limit maximum steps per frame to avoid freeze if audio is slow consuming
        let mut steps_this_frame = 0;
        while tx.len() < 2048 && steps_this_frame < 2000 {
            grid.step(c2, base_damping);

            if step_counter % 100 == 0 {
                // Agent Logic (Runs ~440Hz)
                // Need to split borrow? Agents need mut ref to grid.
                // We are inside loop.
                for agent in &mut agents {
                    agent.update(&mut grid, &monitor);
                }

                // Reproduction / Death
                let mut new_agents = Vec::new();
                agents.retain(|a| a.bio_energy > 0.0);
                for agent in &mut agents {
                    if agent.bio_energy > 200.0 {
                        agent.bio_energy -= 100.0;
                        new_agents.push(agent.clone_mutate(step_counter));
                    }
                }
                agents.append(&mut new_agents);
                if agents.len() < 4 {
                    agents.push(Agent::new_random(step_counter));
                }
            }

            // Sample Audio: Sum of agent inputs (what the hive hears)
            // Or sum of grid pressure at agent locations?
            // Agent `update` reads pressure.
            // Let's sample the pressure at the center of the grid as a "Room Mic"
            // and maybe mix in agent activity?
            // Simple Room Mic at center:
            let center_idx = grid.idx_raw(GRID_SIZE/2, GRID_SIZE/2, GRID_SIZE/2, GRID_SIZE/2);
            let sample = grid.u[center_idx];

            // Send to audio thread
            // Ignore error (if channel closed)
            let _ = tx.send(sample);

            step_counter += 1;
            steps_this_frame += 1;
        }

        // 2. Visualization

        // Input
        if is_mouse_button_pressed(MouseButton::Left) {
             let center = Point4D::new(GRID_SIZE/2, GRID_SIZE/2, GRID_SIZE/2, GRID_SIZE/2);
             grid.pluck(center, 1.0);
        }

        // Rotate Space
        let rot_speed = 0.1 + monitor.load_avg * 0.2;
        angle_xw += dt * rot_speed;
        angle_yw += dt * rot_speed * 0.5;
        angle_zw += dt * 0.05;

        // Camera Control
        if is_key_down(KeyCode::Left) { cam_angle_y += 2.0 * dt; }
        if is_key_down(KeyCode::Right) { cam_angle_y -= 2.0 * dt; }
        if is_key_down(KeyCode::Up) { cam_angle_x += 2.0 * dt; }
        if is_key_down(KeyCode::Down) { cam_angle_x -= 2.0 * dt; }
        if is_key_down(KeyCode::W) { cam_dist -= 5.0 * dt; }
        if is_key_down(KeyCode::S) { cam_dist += 5.0 * dt; }

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

        // Draw Grid
        // Only draw significant points to save FPS
        for (i, p) in visual_grid_points.iter().enumerate() {
            let pressure = grid.u[i];
            let damping = grid.agent_damping[i];

            // Draw if pressure is high OR if damping is active (Agent eating)
            if pressure.abs() > 0.05 || damping < 0.99 {
                let mut v = *p;
                v = v.rotate_xw(angle_xw);
                v = v.rotate_yw(angle_yw);
                v = v.rotate_zw(angle_zw);
                let p3 = v.project_to_3d(4.0);

                let color = if damping < 0.99 {
                    // Being eaten (Damped) -> Blueish
                    BLUE
                } else if pressure > 0.0 {
                    Color::new(1.0, 1.0 - pressure.min(1.0), 1.0 - pressure.min(1.0), 0.8)
                } else {
                    Color::new(1.0 - pressure.abs().min(1.0), 1.0, 1.0, 0.8)
                };

                draw_sphere(p3, 0.05 + pressure.abs() * 0.1, None, color);
            }
        }

        // Draw Agents
        for agent in &agents {
             // Normalized pos -1..1
             let fx = (agent.pos.x as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
             let fy = (agent.pos.y as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
             let fz = (agent.pos.z as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
             let fw = (agent.pos.w as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;

             let mut v = Vec4::new(fx, fy, fz, fw);
             v = v.rotate_xw(angle_xw);
             v = v.rotate_yw(angle_yw);
             v = v.rotate_zw(angle_zw);
             let p3 = v.project_to_3d(4.0);

             draw_sphere(p3, 0.15, None, agent.color);
        }

        set_default_camera();

        draw_text("Chimera Acoustics", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Agents: {}", agents.len()), 10.0, 50.0, 20.0, GREEN);
        draw_text(&format!("CPU (Speed): {:.2}", c2), 10.0, 70.0, 20.0, RED);
        draw_text(&format!("RAM (Damping): {:.2}", base_damping), 10.0, 90.0, 20.0, BLUE);

        next_frame().await
    }
}
