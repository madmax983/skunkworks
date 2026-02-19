mod gen;
mod glyph;
mod starmap;
mod world;

use gen::generate_galaxy;
use macroquad::prelude::*;
use world::{Block, Room, World};

const MOVE_SPEED: f32 = 10.0;
const LOOK_SPEED: f32 = 0.005;

#[macroquad::main("Codex Tardis")]
async fn main() {
    let mut world = generate_galaxy();

    // Capture mouse
    show_mouse(false);
    set_cursor_grab(true);

    loop {
        let delta = get_frame_time();

        // --- Input & Movement ---
        let mouse_delta = mouse_delta_position();
        world.player.yaw -= mouse_delta.x * LOOK_SPEED * 500.0;
        world.player.pitch -= mouse_delta.y * LOOK_SPEED * 500.0;
        world.player.pitch = world.player.pitch.clamp(-1.5, 1.5);

        let front = Vec3::new(world.player.yaw.cos(), 0.0, world.player.yaw.sin()).normalize();
        let right = Vec3::new(world.player.yaw.sin(), 0.0, -world.player.yaw.cos()).normalize();

        let mut move_vec = Vec3::ZERO;
        if is_key_down(KeyCode::W) {
            move_vec += front;
        }
        if is_key_down(KeyCode::S) {
            move_vec -= front;
        }
        if is_key_down(KeyCode::D) {
            move_vec -= right;
        }
        if is_key_down(KeyCode::A) {
            move_vec += right;
        }
        if is_key_down(KeyCode::Space) {
            move_vec += Vec3::Y;
        }
        if is_key_down(KeyCode::LeftShift) {
            move_vec -= Vec3::Y;
        }

        if move_vec.length_squared() > 0.0 {
            move_vec = move_vec.normalize();
            world.player.pos += move_vec * MOVE_SPEED * delta;
        }

        // --- Teleportation Logic ---
        check_teleport(&mut world);

        // --- Rendering ---
        clear_background(BLACK);

        let front = Vec3::new(
            world.player.yaw.cos() * world.player.pitch.cos(),
            world.player.pitch.sin(),
            world.player.yaw.sin() * world.player.pitch.cos(),
        )
        .normalize();

        let camera = Camera3D {
            position: world.player.pos,
            target: world.player.pos + front,
            up: Vec3::Y,
            fovy: 60.0,
            ..Default::default()
        };
        set_camera(&camera);

        if let Some(room) = world.get_room(world.player.current_room_id) {
            draw_room(room);
        }

        set_default_camera();

        // HUD
        draw_text("CODEX TARDIS", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Room: {}", world.player.current_room_id),
            10.0,
            50.0,
            20.0,
            WHITE,
        );
        draw_text(
            "WASD to Move | Mouse to Look",
            10.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}

fn check_teleport(world: &mut World) {
    let player_pos = world.player.pos;
    let room_id = world.player.current_room_id;
    let mut teleport_target: Option<(usize, Vec3)> = None;

    if let Some(room) = world.get_room(room_id) {
        for block in &room.blocks {
            if let Some(target_id) = block.target_room_id {
                let min = block.pos - block.size / 2.0;
                let max = block.pos + block.size / 2.0;

                // Simple AABB check
                if player_pos.x >= min.x
                    && player_pos.x <= max.x
                    && player_pos.y >= min.y
                    && player_pos.y <= max.y
                    && player_pos.z >= min.z
                    && player_pos.z <= max.z
                {
                    teleport_target = Some((target_id, Vec3::ZERO));
                }
            }
        }
    }

    if let Some((target_id, _)) = teleport_target {
        let target_room = world.get_room(target_id).unwrap();
        let target_center = target_room.size / 2.0;
        let entry_pos = Vec3::new(target_center.x, 2.0, target_center.z);
        world.player.current_room_id = target_id;
        world.player.pos = entry_pos;
    }
}

fn draw_room(room: &Room) {
    let s = room.size;
    let tex = room.texture.as_ref();

    // Floor
    draw_plane(
        Vec3::new(s.x / 2.0, 0.0, s.z / 2.0),
        Vec2::new(s.x, s.z),
        tex,
        WHITE,
    );

    // Ceiling
    // draw_plane draws facing up. We need facing down.
    // For now just draw it, it will be visible from below if backface culling is off (Macroquad default is off I think)
    draw_plane(
        Vec3::new(s.x / 2.0, s.y, s.z / 2.0),
        Vec2::new(s.x, s.z),
        tex,
        WHITE,
    );

    // Walls (North, South, East, West)
    // Front Wall (+Z)
    draw_cube(
        Vec3::new(s.x / 2.0, s.y / 2.0, s.z),
        Vec3::new(s.x, s.y, 0.1),
        tex,
        WHITE,
    );

    // Back Wall (0)
    draw_cube(
        Vec3::new(s.x / 2.0, s.y / 2.0, 0.0),
        Vec3::new(s.x, s.y, 0.1),
        tex,
        WHITE,
    );

    // Left Wall (0)
    draw_cube(
        Vec3::new(0.0, s.y / 2.0, s.z / 2.0),
        Vec3::new(0.1, s.y, s.z),
        tex,
        WHITE,
    );

    // Right Wall (+X)
    draw_cube(
        Vec3::new(s.x, s.y / 2.0, s.z / 2.0),
        Vec3::new(0.1, s.y, s.z),
        tex,
        WHITE,
    );

    // Draw Blocks (Portals)
    for block in &room.blocks {
        if block.target_room_id.is_some() {
            // It's a portal / Star Cluster
            draw_cube(block.pos, block.size, None, block.color);
            draw_cube_wires(block.pos, block.size, WHITE);
        } else {
            // Data Star
            draw_sphere(block.pos, 0.5, None, block.color);
        }
    }
}
