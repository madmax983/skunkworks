mod map;
mod render;

use macroquad::models::{draw_mesh, Mesh, Vertex};
use macroquad::prelude::*;
use map::generate_heap;
use render::{draw_room, render_portal_view, PortalRenderer};

#[macroquad::main("Memory Palace")]
async fn main() {
    let mut graph = generate_heap();
    let mut renderer = PortalRenderer::new();

    // Player State
    // Start in Room 0 (Stack)
    let mut pos = Vec3::new(0.0, 0.0, 0.0);
    let mut prev_pos = pos;
    let mut velocity;
    let mut rot = Vec2::new(0.0, 0.0); // Yaw, Pitch
    let mut current_room = 0;

    // Grab Mouse
    // set_cursor_grab(true); // Can be annoying during dev, enable if robust
    show_mouse(false);

    loop {
        let dt = get_frame_time();

        // --- Input ---
        let mouse_delta = mouse_delta_position();
        rot.x -= mouse_delta.x * 0.005; // Yaw
        rot.y += mouse_delta.y * 0.005; // Pitch
        rot.y = rot.y.clamp(-1.5, 1.5);

        // Movement
        let forward = Vec3::new(rot.x.sin(), 0.0, rot.x.cos());
        let right = Vec3::new(rot.x.cos(), 0.0, -rot.x.sin());
        let mut move_dir = Vec3::ZERO;

        if is_key_down(KeyCode::W) {
            move_dir += forward;
        }
        if is_key_down(KeyCode::S) {
            move_dir -= forward;
        }
        if is_key_down(KeyCode::D) {
            move_dir += right;
        }
        if is_key_down(KeyCode::A) {
            move_dir -= right;
        }
        if is_key_down(KeyCode::Space) {
            move_dir += Vec3::Y;
        }
        if is_key_down(KeyCode::LeftShift) {
            move_dir -= Vec3::Y;
        }

        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
        }

        velocity = move_dir * 10.0; // Speed
        pos += velocity * dt;

        // --- Portal Logic (Teleport) ---
        let room = &graph.rooms[current_room];
        let mut teleported = false;

        for portal in &room.portals {
            let p_room = pos - room.pos;
            let p_prev_room = prev_pos - room.pos;

            let inv_rot = portal.rot.inverse();
            let p_local = inv_rot * (p_room - portal.pos);
            let p_prev_local = inv_rot * (p_prev_room - portal.pos);

            // Crossing check: Z goes from >0 (front) to <=0 (back)
            if p_prev_local.z > 0.0 && p_local.z <= 0.0 {
                let half_w = portal.size.x / 2.0;
                let half_h = portal.size.y / 2.0;

                if p_local.x.abs() < half_w && p_local.y.abs() < half_h {
                    // TELEPORT!
                    let dest_room = &graph.rooms[portal.target_room];
                    let dest_portal = &dest_room.portals[portal.target_portal];

                    let m_src = Mat4::from_translation(room.pos)
                        * Mat4::from_translation(portal.pos)
                        * Mat4::from_quat(portal.rot);
                    let m_dest = Mat4::from_translation(dest_room.pos)
                        * Mat4::from_translation(dest_portal.pos)
                        * Mat4::from_quat(dest_portal.rot);
                    let flip = Mat4::from_rotation_y(std::f32::consts::PI);

                    let transform = m_dest * flip * m_src.inverse();

                    pos = transform.transform_point3(pos);
                    prev_pos = transform.transform_point3(prev_pos);

                    // Rotate View
                    let q_src = portal.rot;
                    let q_dest = dest_portal.rot;
                    let q_flip = Quat::from_rotation_y(std::f32::consts::PI);
                    let q_diff = q_dest * q_flip * q_src.inverse();

                    let current_forward = Vec3::new(rot.x.sin(), 0.0, rot.x.cos());
                    let new_forward = q_diff * current_forward;
                    rot.x = new_forward.x.atan2(new_forward.z);

                    current_room = portal.target_room;
                    teleported = true;
                    // println!("Teleported to Room {}", current_room);
                    break;
                }
            }
        }

        if !teleported {
            prev_pos = pos;
        }

        // --- Render ---

        // 1. Render Portal Views
        let mut portal_textures = Vec::new();
        let room = &graph.rooms[current_room];

        let cam_target = pos + Vec3::new(rot.x.sin(), rot.y.sin(), rot.x.cos());

        for (i, portal) in room.portals.iter().enumerate() {
            let tex = render_portal_view(
                &mut renderer,
                &graph,
                current_room,
                portal,
                pos,
                cam_target,
                Vec3::Y,
                1, // Depth
                i, // Target Index
                vec2(screen_width(), screen_height()),
            );
            portal_textures.push(tex);
        }

        // 2. Main Render
        set_default_camera();
        clear_background(BLACK);

        set_camera(&Camera3D {
            position: pos,
            target: cam_target,
            up: Vec3::Y,
            fovy: 60.0,
            ..Default::default()
        });

        draw_room(&graph, current_room);

        // Draw Portals
        let room = &graph.rooms[current_room];
        for (i, portal) in room.portals.iter().enumerate() {
            if let Some(tex) = &portal_textures[i] {
                // Portal Quad in World Space
                let portal_pos = room.pos + portal.pos;
                draw_portal_quad(portal_pos, portal.rot, portal.size, Some(tex.clone()));
            } else {
                // Draw Black Quad if no texture (depth limit)
                let portal_pos = room.pos + portal.pos;
                draw_portal_quad(portal_pos, portal.rot, portal.size, None);
            }
        }

        // UI
        set_default_camera();
        draw_text("Memory Palace", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Room: {}", current_room), 10.0, 60.0, 20.0, GRAY);
        draw_text(
            "WASD Move | Space/Shift Up/Down",
            10.0,
            screen_height() - 20.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}

fn draw_portal_quad(pos: Vec3, rot: Quat, size: Vec2, texture: Option<Texture2D>) {
    let half_w = size.x / 2.0;
    let half_h = size.y / 2.0;

    // Vertices in local space (Z=0)
    let v0 = vec3(-half_w, -half_h, 0.0);
    let v1 = vec3(half_w, -half_h, 0.0);
    let v2 = vec3(half_w, half_h, 0.0);
    let v3 = vec3(-half_w, half_h, 0.0);

    let transform = Mat4::from_translation(pos) * Mat4::from_quat(rot);

    let p0 = transform.transform_point3(v0);
    let p1 = transform.transform_point3(v1);
    let p2 = transform.transform_point3(v2);
    let p3 = transform.transform_point3(v3);

    // UVs: 0,0 is bottom-left
    let vertices = vec![
        Vertex {
            position: p0,
            uv: vec2(0.0, 0.0),
            color: WHITE.into(),
            normal: vec4(0.0, 0.0, 1.0, 0.0),
        }, // BL
        Vertex {
            position: p1,
            uv: vec2(1.0, 0.0),
            color: WHITE.into(),
            normal: vec4(0.0, 0.0, 1.0, 0.0),
        }, // BR
        Vertex {
            position: p2,
            uv: vec2(1.0, 1.0),
            color: WHITE.into(),
            normal: vec4(0.0, 0.0, 1.0, 0.0),
        }, // TR
        Vertex {
            position: p3,
            uv: vec2(0.0, 1.0),
            color: WHITE.into(),
            normal: vec4(0.0, 0.0, 1.0, 0.0),
        }, // TL
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];

    let mesh = Mesh {
        vertices,
        indices,
        texture,
    };

    draw_mesh(&mesh);
}
