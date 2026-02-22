mod agent;
mod math;
mod monitor;

use agent::Agent;
use math::Vec4D;
use monitor::SystemMonitor;

use macroquad::prelude::*;

const AGENT_COUNT: usize = 64;

#[macroquad::main("Hyper-Chimera")]
async fn main() {
    let mut monitor = SystemMonitor::new();
    let mut agents: Vec<Agent> = (0..AGENT_COUNT).map(|_| Agent::new()).collect();

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 10.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    loop {
        // Update
        monitor.update();
        let dt = get_frame_time();

        for agent in &mut agents {
            agent.update(&monitor);
        }

        // Rotate Space
        angle_xw += dt * 0.1;
        angle_yw += dt * 0.05;
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

        // Distortion
        let sx = 1.0 + monitor.cpu_usage * 5.0;
        let sy = 1.0 + monitor.mem_usage * 5.0;
        let sz = 1.0 + monitor.swap_usage * 5.0;
        let sw = 1.0 + monitor.load_avg;

        // Draw Tesseract
        draw_tesseract_wireframe(
            Vec4D::new(4.0, 4.0, 4.0, 4.0),
            angle_xw,
            angle_yw,
            angle_zw,
            sx,
            sy,
            sz,
            sw,
        );

        // Draw Agents
        let transform = |v: Vec4D| -> Vec3 {
            let mut v = v.scale_dim(sx, sy, sz, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            v.project_to_3d(6.0)
        };

        for agent in &agents {
            let p = transform(agent.pos);
            draw_sphere(p, 0.1, None, agent.color);
        }

        set_default_camera();
        draw_text("Hyper-Chimera", 10.0, 20.0, 30.0, WHITE);
        draw_text("Relativistic Metabolism Active", 10.0, 40.0, 20.0, GRAY);

        draw_text(
            &format!("CPU (X-Stretch): {:.2}", sx),
            10.0,
            70.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("MEM (Y-Stretch): {:.2}", sy),
            10.0,
            90.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("Agents: {}", agents.len()),
            10.0,
            110.0,
            20.0,
            GREEN,
        );

        next_frame().await
    }
}

fn draw_tesseract_wireframe(
    bounds: Vec4D,
    axw: f32,
    ayw: f32,
    azw: f32,
    sx: f32,
    sy: f32,
    sz: f32,
    sw: f32,
) {
    let mut verts = Vec::new();
    for i in 0..16 {
        let x = if i & 1 != 0 { bounds.x } else { -bounds.x };
        let y = if i & 2 != 0 { bounds.y } else { -bounds.y };
        let z = if i & 4 != 0 { bounds.z } else { -bounds.z };
        let w = if i & 8 != 0 { bounds.w } else { -bounds.w };
        verts.push(Vec4D::new(x, y, z, w));
    }

    let transform = |v: Vec4D| -> Vec3 {
        let mut v = v.scale_dim(sx, sy, sz, 1.0 + sw);
        v = v.rotate_xw(axw);
        v = v.rotate_yw(ayw);
        v = v.rotate_zw(azw);
        v.project_to_3d(6.0)
    };

    for i in 0..16 {
        for j in (i + 1)..16 {
            let diff: usize = i ^ j;
            if diff.count_ones() == 1 {
                let p1 = transform(verts[i]);
                let p2 = transform(verts[j]);
                draw_line_3d(p1, p2, Color::new(0.5, 0.5, 0.5, 0.2));
            }
        }
    }
}
