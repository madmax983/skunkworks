use macroquad::prelude::*;
use projective_walk::topology::{Point, step};
use projective_walk::surface::{generate_mesh, roman_surface};

struct Collectible {
    u: f32,
    v: f32,
    active: bool,
}

#[macroquad::main("Projective Walk")]
async fn main() {
    let (vertices, indices) = generate_mesh(50);

    let mesh = Mesh {
        vertices,
        indices,
        texture: None,
    };

    let mut player = Point::new(0.5, 0.5, Vec2::new(0.0, 1.0)); // Facing Up

    let mut cam_yaw = 0.0f32;
    let mut cam_pitch = 0.5f32;
    let mut cam_dist = 10.0f32;

    let mut trail: std::collections::VecDeque<Vec3> = std::collections::VecDeque::new();

    let mut score = 0;
    let mut collectibles = Vec::new();
    for _ in 0..10 {
        collectibles.push(Collectible {
            u: macroquad::rand::gen_range(0.0, 1.0),
            v: macroquad::rand::gen_range(0.0, 1.0),
            active: true,
        });
    }

    loop {
        // Input
        let mut delta = Vec2::ZERO;
        let speed = 0.01;

        if is_key_down(KeyCode::W) {
            delta += player.facing * speed;
        }
        if is_key_down(KeyCode::S) {
            delta -= player.facing * speed;
        }

        let rot_speed: f32 = 0.05;
        if is_key_down(KeyCode::A) {
            let (sin, cos) = rot_speed.sin_cos();
            let new_x = player.facing.x * cos - player.facing.y * sin;
            let new_y = player.facing.x * sin + player.facing.y * cos;
            player.facing = vec2(new_x, new_y).normalize();
        }
        if is_key_down(KeyCode::D) {
            let (sin, cos) = (-rot_speed).sin_cos();
            let new_x = player.facing.x * cos - player.facing.y * sin;
            let new_y = player.facing.x * sin + player.facing.y * cos;
            player.facing = vec2(new_x, new_y).normalize();
        }

        if delta != Vec2::ZERO {
            player = step(player, delta);

            trail.push_back(roman_surface(player.u, player.v));
            if trail.len() > 200 {
                trail.pop_front();
            }
        }

        // Collectibles
        let p_pos_3d = roman_surface(player.u, player.v);
        for c in collectibles.iter_mut() {
            if c.active {
                let c_pos = roman_surface(c.u, c.v);
                if p_pos_3d.distance(c_pos) < 0.5 {
                    c.active = false;
                    score += 1;
                }
            }
        }

        if collectibles.iter().all(|c| !c.active) {
            // Respawn
            for c in collectibles.iter_mut() {
                c.u = macroquad::rand::gen_range(0.0, 1.0);
                c.v = macroquad::rand::gen_range(0.0, 1.0);
                c.active = true;
            }
        }

        // Camera
        if is_mouse_button_down(MouseButton::Right) {
            let d = mouse_delta_position();
            cam_yaw += d.x * 2.0;
            cam_pitch += d.y * 2.0;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }

        let scroll = mouse_wheel().1;
        cam_dist = (cam_dist - scroll * 1.0).clamp(2.0, 50.0);

        // Render
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        let cam_pos = vec3(
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_mesh(&mesh);

        // Draw Collectibles
        for c in collectibles.iter() {
            if c.active {
                draw_sphere(roman_surface(c.u, c.v), 0.2, None, GREEN);
            }
        }

        let p_pos = roman_surface(player.u, player.v);
        draw_sphere(p_pos, 0.15, None, RED);

        let eps = 0.01;
        let p_next = roman_surface(player.u + player.facing.x * eps, player.v + player.facing.y * eps);
        let facing_3d = (p_next - p_pos).normalize_or_zero();
        draw_line_3d(p_pos, p_pos + facing_3d * 0.5, YELLOW);

        if trail.len() > 1 {
            let points: Vec<Vec3> = trail.iter().cloned().collect();
            for i in 0..points.len()-1 {
                draw_line_3d(points[i], points[i+1], Color::new(1.0, 0.5, 0.0, 0.5));
            }
        }

        draw_grid(20, 1.0, BLACK, GRAY);

        set_default_camera();
        draw_text("Projective Walk", 10.0, 30.0, 30.0, WHITE);
        draw_text("WASD to move, Right Mouse to rotate camera", 10.0, 50.0, 20.0, GRAY);
        draw_text(&format!("Score: {}", score), 10.0, 70.0, 30.0, GOLD);

        next_frame().await
    }
}
