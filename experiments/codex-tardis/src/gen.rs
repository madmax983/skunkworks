use crate::starmap::StarMap;
use crate::world::{Block, Player, Room, World};
use ::rand::prelude::*;
use macroquad::prelude::*;

const MAX_DEPTH: usize = 3;
const ROOM_SCALE_FACTOR: f32 = 5.0;

pub fn generate_galaxy() -> World {
    let mut world = World::new();
    let mut rng = ::rand::thread_rng();

    // Create Root Room (The Galaxy Core)
    let root_size = Vec3::new(50.0, 20.0, 50.0);
    // Dark background for space
    let mut root_room = Room::new(0, root_size, BLACK);

    let mut next_room_id = 1;
    generate_recursive(&mut world, &mut root_room, &mut next_room_id, 0, &mut rng);

    world.add_room(root_room);

    // Position player in center of root room
    world.player = Player::new(Vec3::new(root_size.x / 2.0, 2.0, root_size.z / 2.0), 0);

    world
}

fn generate_recursive(
    world: &mut World,
    current_room: &mut Room,
    next_room_id: &mut usize,
    depth: usize,
    rng: &mut ThreadRng,
) {
    // Generate Codex Texture (Simulated Payload)
    // In a real app, this would be the file content.
    // Here we generate a random string based on ID and Depth.
    let payload = format!(
        "Room {} Depth {} - Codex Data: {}",
        current_room.id,
        depth,
        rng.gen::<u64>()
    );
    let starmap = StarMap::new(payload.as_bytes(), 16);
    // Note: We cannot generate texture here because we might not have GL context if run in test?
    // But this is run in main(). So it is fine.
    // However, storing Texture2D requires Clone which is fine (RefCell internally).
    // Let's defer texture generation to rendering or just do it here if possible.
    // We'll set it here assuming GL is active.
    current_room.texture = Some(starmap.generate_texture());

    if depth >= MAX_DEPTH {
        return;
    }

    let num_blocks = rng.gen_range(3..8);
    for _ in 0..num_blocks {
        // Random Position
        let w = current_room.size.x;
        let d = current_room.size.z;
        let pos = Vec3::new(
            rng.gen_range(5.0..w - 5.0),
            rng.gen_range(2.0..10.0), // Floating
            rng.gen_range(5.0..d - 5.0),
        );

        let size = Vec3::new(2.0, 2.0, 2.0); // Star Clusters are small cubes

        let is_struct = rng.gen_bool(0.6);

        // Portal Color (Nebula-ish)
        let color = Color::new(rng.gen_range(0.5..1.0), rng.gen_range(0.0..0.5), 1.0, 0.8);

        if is_struct {
            let inner_room_id = *next_room_id;
            *next_room_id += 1;

            let inner_w = size.x * ROOM_SCALE_FACTOR;
            let inner_h = size.y * ROOM_SCALE_FACTOR * 2.0;
            let inner_d = size.z * ROOM_SCALE_FACTOR;
            let inner_size = Vec3::new(inner_w, inner_h, inner_d);

            let mut inner_room = Room::new(inner_room_id, inner_size, BLACK);

            generate_recursive(world, &mut inner_room, next_room_id, depth + 1, rng);
            world.add_room(inner_room);

            let block = Block {
                pos,
                size,
                color,
                target_room_id: Some(inner_room_id),
                portal_face_normal: Vec3::Z,
            };
            current_room.add_block(block);
        } else {
            // "Data Star" (Non-portal)
            let block = Block {
                pos,
                size: Vec3::ONE,
                color: YELLOW,
                target_room_id: None,
                portal_face_normal: Vec3::ZERO,
            };
            current_room.add_block(block);
        }
    }
}
