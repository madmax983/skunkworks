mod kinematics;

use macroquad::prelude::*;
use ::rand::Rng;
use kinematics::MiuraGrid;

struct Ant {
    u: f32,
    v: f32,
    state: AntState,
    color: Color,
}

#[derive(Clone, Copy)]
enum AntState {
    Walking { target_u: f32, target_v: f32 },
    Jumping { start_u: f32, start_v: f32, end_u: f32, end_v: f32, progress: f32 },
}

fn get_pos_bilinear(grid: &MiuraGrid, vertices: &[Vec3], u: f32, v: f32) -> Vec3 {
    let width = grid.cols + 1;
    let u_clamped = u.clamp(0.0, grid.cols as f32 - 0.001);
    let v_clamped = v.clamp(0.0, grid.rows as f32 - 0.001);

    let u_int = u_clamped.floor() as usize;
    let v_int = v_clamped.floor() as usize;
    let u_frac = u_clamped - u_int as f32;
    let v_frac = v_clamped - v_int as f32;

    let idx00 = v_int * width + u_int;
    let idx10 = v_int * width + u_int + 1;
    let idx01 = (v_int + 1) * width + u_int;
    let idx11 = (v_int + 1) * width + u_int + 1;

    let p00 = vertices[idx00];
    let p10 = vertices[idx10];
    let p01 = vertices[idx01];
    let p11 = vertices[idx11];

    let p0 = p00.lerp(p10, u_frac);
    let p1 = p01.lerp(p11, u_frac);

    p0.lerp(p1, v_frac)
}

#[macroquad::main("Folded Colony")]
async fn main() {
    let grid = MiuraGrid::new(20, 15);
    let mut rng = ::rand::thread_rng();

    // Camera state
    let mut cam_yaw: f32 = 0.5;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 40.0;
    let mut last_mouse_pos = mouse_position();

    // Deployment state
    let mut expansion = 0.1; // Start folded
    let mut deploying = false; // Start static to show folded state
    let deploy_speed = 0.2;
    let mut auto_oscillate = false;

    // Ants
    let mut ants: Vec<Ant> = (0..200).map(|_| {
        let u = rng.gen_range(0.0..grid.cols as f32);
        let v = rng.gen_range(0.0..grid.rows as f32);
        Ant {
            u,
            v,
            state: AntState::Walking {
                target_u: (u + rng.gen_range(-1.0..1.0)).clamp(0.0, grid.cols as f32),
                target_v: (v + rng.gen_range(-1.0..1.0)).clamp(0.0, grid.rows as f32),
            },
            color: if rng.gen_bool(0.5) { RED } else { BLUE },
        }
    }).collect();

    loop {
        let dt = get_frame_time();

        // --- Input ---
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let delta = vec2(
                mouse_pos.0 - last_mouse_pos.0,
                mouse_pos.1 - last_mouse_pos.1,
            );
            cam_yaw -= delta.x * 0.01;
            cam_pitch += delta.y * 0.01;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }
        last_mouse_pos = mouse_position();

        let wheel = mouse_wheel().1;
        cam_dist -= wheel * 0.1 * cam_dist;
        cam_dist = cam_dist.clamp(5.0, 150.0);

        if is_key_down(KeyCode::Right) {
            expansion = (expansion + 1.0 * dt).min(1.0);
            deploying = false;
        }
        if is_key_down(KeyCode::Left) {
            expansion = (expansion - 1.0 * dt).max(0.01);
            deploying = false;
        }
        if is_key_pressed(KeyCode::Space) {
            deploying = !deploying;
            auto_oscillate = false;
        }
        if is_key_pressed(KeyCode::O) {
            auto_oscillate = !auto_oscillate;
            deploying = false;
        }

        // --- Logic ---
        if deploying {
            expansion += deploy_speed * dt;
            if expansion >= 1.0 {
                expansion = 1.0;
                deploying = false;
            }
        }
        if auto_oscillate {
            expansion = (get_time() as f32 * 0.5).sin() * 0.45 + 0.55; // Oscillate 0.1 to 1.0
        }

        let vertices = grid.get_vertices(expansion);

        // Update Ants
        for ant in &mut ants {
            match ant.state {
                AntState::Walking { target_u, target_v } => {
                    let speed = 2.0 * dt;
                    let du = target_u - ant.u;
                    let dv = target_v - ant.v;
                    let dist = (du * du + dv * dv).sqrt();

                    if dist < speed {
                        ant.u = target_u;
                        ant.v = target_v;
                        // Pick new target (neighbor)
                        // Simple random walk
                        let next_u = (ant.u + rng.gen_range(-1.0..1.0)).clamp(0.0, grid.cols as f32);
                        let next_v = (ant.v + rng.gen_range(-1.0..1.0)).clamp(0.0, grid.rows as f32);
                        ant.state = AntState::Walking { target_u: next_u, target_v: next_v };
                    } else {
                        ant.u += du / dist * speed;
                        ant.v += dv / dist * speed;
                    }

                    // Wormhole check
                    // Only if folded enough
                    if expansion < 0.4 && rng.gen_bool(0.01) {
                        // Try to find a jump target
                        let jump_u = rng.gen_range(0.0..grid.cols as f32);
                        let jump_v = rng.gen_range(0.0..grid.rows as f32);

                        // Topological distance (UV space)
                        let dist_uv = ((jump_u - ant.u).powi(2) + (jump_v - ant.v).powi(2)).sqrt();

                        if dist_uv > 5.0 { // Must be far in UV space
                            let pos_curr = get_pos_bilinear(&grid, &vertices, ant.u, ant.v);
                            let pos_jump = get_pos_bilinear(&grid, &vertices, jump_u, jump_v);

                            if pos_curr.distance(pos_jump) < 1.5 { // But close in 3D space
                                // Initiate Jump!
                                ant.state = AntState::Jumping {
                                    start_u: ant.u,
                                    start_v: ant.v,
                                    end_u: jump_u,
                                    end_v: jump_v,
                                    progress: 0.0,
                                };
                            }
                        }
                    }
                }
                AntState::Jumping { start_u: _, start_v: _, end_u, end_v, ref mut progress } => {
                    *progress += 2.0 * dt; // Jump speed
                    if *progress >= 1.0 {
                        ant.u = end_u;
                        ant.v = end_v;
                        ant.state = AntState::Walking {
                            target_u: (end_u + rng.gen_range(-1.0..1.0)).clamp(0.0, grid.cols as f32),
                            target_v: (end_v + rng.gen_range(-1.0..1.0)).clamp(0.0, grid.rows as f32),
                        };
                    } else {
                        // Interpolate u, v for rendering? No, we render jump arc in 3D.
                        // But we update internal state to match start for now.
                    }
                }
            }
        }

        // --- Render ---
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        let cam_pos = vec3(
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(grid.cols as f32 * grid.params.a * 0.5, grid.rows as f32 * grid.params.b * 0.5, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Draw Mesh
        let width = grid.cols + 1;
        for j in 0..grid.rows {
            for i in 0..grid.cols {
                let idx0 = j * width + i;
                let idx1 = j * width + i + 1;
                let idx2 = (j + 1) * width + i + 1;
                let idx3 = (j + 1) * width + i;

                let v0 = vertices[idx0];
                let v1 = vertices[idx1];
                let v2 = vertices[idx2];
                let v3 = vertices[idx3];

                // Simple wireframe
                let color = Color::new(0.3, 0.3, 0.3, 0.5);
                draw_line_3d(v0, v1, color);
                draw_line_3d(v1, v2, color);
                draw_line_3d(v2, v3, color);
                draw_line_3d(v3, v0, color);
            }
        }

        // Draw Ants
        for ant in &ants {
            match ant.state {
                AntState::Walking { .. } => {
                    let pos = get_pos_bilinear(&grid, &vertices, ant.u, ant.v);
                    draw_sphere(pos, 0.15, None, ant.color);
                }
                AntState::Jumping { start_u, start_v, end_u, end_v, progress } => {
                    let start_pos = get_pos_bilinear(&grid, &vertices, start_u, start_v);
                    let end_pos = get_pos_bilinear(&grid, &vertices, end_u, end_v);

                    // Linear interpolation for jump
                    let current_pos = start_pos.lerp(end_pos, progress);

                    // Draw Jump Arc line
                    draw_line_3d(start_pos, end_pos, YELLOW);
                    draw_sphere(current_pos, 0.2, None, YELLOW);
                }
            }
        }

        set_default_camera();
        draw_text("Folded Colony 🐜", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Expansion: {:.2}", expansion), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("Space: Deploy | O: Oscillate | Arrows: Manual", 10.0, 70.0, 20.0, GRAY);

        let jump_count = ants.iter().filter(|a| matches!(a.state, AntState::Jumping { .. })).count();
        if jump_count > 0 {
             draw_text(&format!("ACTIVE JUMPS: {}", jump_count), 10.0, 90.0, 20.0, YELLOW);
        }

        next_frame().await
    }
}
