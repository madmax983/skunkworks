mod mesh_gen;

use ::rand::prelude::*;
use macroquad::prelude::*;
use rayon::prelude::*;

use mesh_gen::generate_miura_ori;
use physics_pbd::Constraint;

// DDoS Swarm logic
pub const AGENT_COUNT: usize = 10_000;
pub const WORLD_SIZE: f32 = 40.0;
pub const SPEED: f32 = 10.0;
pub const SERVER_POS: Vec2 = Vec2::new(0.0, 0.0);

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
}

#[macroquad::main("Origami DDoS")]
async fn main() {
    if std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux") {
        return;
    }

    let rows = 12;
    let cols = 12;
    let mut mesh_data = generate_miura_ori(rows, cols);

    let mut agents = Vec::with_capacity(AGENT_COUNT);
    let mut rng = ::rand::thread_rng();

    for _ in 0..AGENT_COUNT {
        // Spawn randomly around the edges
        let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
        let r = WORLD_SIZE * 0.8;
        let pos = vec2(angle.cos() * r, angle.sin() * r);
        let target_dir = (SERVER_POS - pos).normalize_or_zero();
        agents.push(Locust {
            pos,
            vel: target_dir * SPEED,
        });
    }

    let firewalls = vec![
        (vec2(10.0, 0.0), 5.0),
        (vec2(-10.0, 0.0), 5.0),
        (vec2(0.0, 10.0), 5.0),
        (vec2(0.0, -10.0), 5.0),
    ];

    let camera = Camera3D {
        position: vec3(0.0, 30.0, 40.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    loop {
        let dt = get_frame_time().min(0.05);

        // Update Swarm
        agents.par_iter_mut().for_each(|agent| {
            // Seek center
            let mut desired = (SERVER_POS - agent.pos).normalize_or_zero() * SPEED;

            // Avoid firewalls
            for (fw_pos, fw_rad) in &firewalls {
                let d = agent.pos.distance(*fw_pos);
                if d < *fw_rad + 2.0 {
                    let avoid = (agent.pos - *fw_pos).normalize_or_zero() * SPEED * 2.0;
                    desired += avoid;
                }
            }

            agent.vel = agent.vel.lerp(desired, dt * 2.0).normalize_or_zero() * SPEED;
            agent.pos += agent.vel * dt;

            // Reset if they reach center
            if agent.pos.distance(SERVER_POS) < 2.0 {
                let mut rng = ::rand::thread_rng();
                let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
                let r = WORLD_SIZE * 0.8;
                agent.pos = vec2(angle.cos() * r, angle.sin() * r);
            }
        });

        // Compute heat map from agents
        // Update origami constraints based on local agent density
        for c_idx in 0..mesh_data.system.constraints.len() {
            if let Constraint::Actuator {
                p1,
                p2,
                min_len: _,
                max_len: _,
                ref mut factor,
                ..
            } = mesh_data.system.constraints[c_idx]
            {
                let p1_pos = mesh_data.system.particles[p1].pos;
                let p2_pos = mesh_data.system.particles[p2].pos;

                let mid = (p1_pos + p2_pos) * 0.5;
                // Map mid (X, Y) to 2D space
                let mid_2d = vec2(mid.x, mid.y); // Note origami z is height

                // Count agents near this constraint
                let mut local_density = 0.0_f32;
                for a in &agents {
                    let d = a.pos.distance(mid_2d);
                    if d < 3.0 {
                        local_density += 1.0;
                    }
                }

                // If density is high (under attack), crumple (factor -> 0.0)
                // If density is low, expand (factor -> 1.0)
                let target_factor = (1.0 - (local_density / 50.0)).clamp(0.05, 1.0);
                *factor = *factor * 0.9 + target_factor * 0.1;
            }
        }

        // PBD step
        mesh_data.system.step(0.016, 5);

        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));
        set_camera(&camera);

        // Draw Swarm (mapped to 3D)
        for a in &agents {
            draw_cube(vec3(a.pos.x, a.pos.y, 1.0), vec3(0.2, 0.2, 0.2), None, RED);
        }

        // Draw Firewalls
        for (fw_pos, fw_rad) in &firewalls {
            draw_sphere(
                vec3(fw_pos.x, fw_pos.y, 0.5),
                *fw_rad,
                None,
                Color::new(0.0, 0.5, 1.0, 0.3),
            );
        }

        // Draw Target
        draw_sphere(vec3(SERVER_POS.x, SERVER_POS.y, 0.5), 2.0, None, GREEN);

        // Draw Mesh
        for i in (0..mesh_data.indices.len()).step_by(3) {
            let i0 = mesh_data.indices[i] as usize;
            let i1 = mesh_data.indices[i + 1] as usize;
            let i2 = mesh_data.indices[i + 2] as usize;

            let p0 = mesh_data.system.particles[i0].pos;
            let p1 = mesh_data.system.particles[i1].pos;
            let p2 = mesh_data.system.particles[i2].pos;

            // Compute normal
            let v1 = p1 - p0;
            let v2 = p2 - p0;
            let mut normal = v1.cross(v2).normalize_or_zero();
            if normal.z < 0.0 {
                normal = -normal;
            }

            // Simple lighting
            let light_dir = vec3(0.5, 0.5, 1.0).normalize();
            let diff = normal.dot(light_dir).max(0.1);

            let base_color = vec3(0.8, 0.8, 0.9);
            let c = base_color * diff;
            let color = Color::new(c.x, c.y, c.z, 1.0);

            draw_line_3d(p0, p1, color);
            draw_line_3d(p1, p2, color);
            draw_line_3d(p2, p0, color);
        }

        set_default_camera();
        draw_text("Origami DDoS", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 60.0, 30.0, LIGHTGRAY);

        next_frame().await
    }
}
