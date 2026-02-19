mod gen;
mod world;

use gen::generate_heap;
use macroquad::prelude::*;
use world::{Block, Room, World};

const MOVE_SPEED: f32 = 10.0;
const LOOK_SPEED: f32 = 0.005;

#[macroquad::main("Tardis Memory")]
async fn main() {
    let mut world = generate_heap();

    // Portal Rendering Resources
    let max_portals = 10;
    let mut render_targets: Vec<RenderTarget> = Vec::new();
    for _ in 0..max_portals {
        let target = render_target(512, 512);
        target.texture.set_filter(FilterMode::Linear);
        render_targets.push(target);
    }

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

        // 1. Render Portals to Textures
        let current_room_id = world.player.current_room_id;
        let mut portal_textures: Vec<(usize, Texture2D)> = Vec::new(); // (block_index, texture)

        if let Some(room) = world.get_room(current_room_id) {
            let mut target_idx = 0;

            for (block_idx, block) in room.blocks.iter().enumerate() {
                if let Some(target_id) = block.target_room_id {
                    if target_idx >= render_targets.len() {
                        break;
                    }

                    let target = render_targets[target_idx].clone();
                    target_idx += 1;

                    let target_room = world.get_room(target_id).unwrap();
                    let target_center = target_room.size / 2.0;

                    let cam_pos = Vec3::new(
                        target_center.x,
                        target_center.y + 2.0,
                        target_center.z - 5.0,
                    );
                    let cam_target = Vec3::new(
                        target_center.x,
                        target_center.y + 2.0,
                        target_center.z + 5.0,
                    );

                    let mut portal_cam = Camera3D {
                        position: cam_pos,
                        target: cam_target,
                        up: Vec3::Y,
                        fovy: 60.0,
                        aspect: Some(1.0),
                        ..Default::default()
                    };
                    portal_cam.render_target = Some(target.clone());

                    set_camera(&portal_cam);
                    clear_background(target_room.background_color);
                    draw_room_internal(&world, target_id);
                    set_default_camera();

                    portal_textures.push((block_idx, target.texture.clone()));
                }
            }
        }

        // 2. Render Main View
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
            draw_room_walls(room);
            for (i, block) in room.blocks.iter().enumerate() {
                let texture_ref = portal_textures
                    .iter()
                    .find(|(idx, _)| *idx == i)
                    .map(|(_, t)| t);
                draw_block(block, texture_ref);
            }
        }

        set_default_camera();
        draw_text(
            "WASD+Space/Shift to Move, Mouse to Look",
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Current Room: {}", world.player.current_room_id),
            10.0,
            60.0,
            20.0,
            WHITE,
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

fn draw_room_walls(room: &Room) {
    let s = room.size;
    draw_grid(20, 1.0, BLACK, GRAY);
    draw_cube(
        Vec3::new(s.x / 2.0, -0.5, s.z / 2.0),
        Vec3::new(s.x, 1.0, s.z),
        None,
        room.background_color,
    );
    draw_cube_wires(Vec3::new(s.x / 2.0, s.y / 2.0, s.z / 2.0), s, WHITE);
}

fn draw_room_internal(world: &World, room_id: usize) {
    if let Some(room) = world.get_room(room_id) {
        draw_room_walls(room);
        for block in &room.blocks {
            draw_block(block, None);
        }
    }
}

fn draw_block(block: &Block, portal_texture: Option<&Texture2D>) {
    if let Some(texture) = portal_texture {
        draw_cube(block.pos, block.size, None, block.color);
        let face_pos = block.pos + Vec3::new(0.0, 0.0, block.size.z / 2.0 + 0.05);
        let plate_size = Vec3::new(block.size.x * 0.9, block.size.y * 0.9, 0.05);
        draw_cube(face_pos, plate_size, Some(texture), WHITE);
    } else {
        draw_cube(block.pos, block.size, None, block.color);
        draw_cube_wires(block.pos, block.size, BLACK);
    }
}
