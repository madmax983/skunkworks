use crate::world::{Block, Player, Room, World};
use macroquad::prelude::*;
// Use the rand crate explicitly
use ::rand::prelude::*;
use ::rand::rngs::ThreadRng;

const MAX_DEPTH: usize = 3;
const ROOM_SCALE_FACTOR: f32 = 5.0; // Inner room is this much bigger than outer block

pub fn generate_heap() -> World {
    let mut world = World::new();
    let mut rng = ::rand::thread_rng();

    // Create Root Room (The Heap)
    let root_size = Vec3::new(50.0, 20.0, 50.0);
    let mut root_room = Room::new(0, root_size, DARKGRAY);

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
    if depth >= MAX_DEPTH {
        return;
    }

    let num_blocks = rng.gen_range(3..8);
    for _ in 0..num_blocks {
        // Random Position in Room (avoiding walls)
        let w = current_room.size.x;
        let d = current_room.size.z;
        let pos = Vec3::new(
            rng.gen_range(2.0..w - 2.0),
            0.0,
            rng.gen_range(2.0..d - 2.0),
        );

        let size = Vec3::new(
            rng.gen_range(1.0..3.0),
            rng.gen_range(1.0..3.0),
            rng.gen_range(1.0..3.0),
        );

        let is_struct = rng.gen_bool(0.4); // 40% chance to be a struct with inner room

        let color = Color::new(rng.gen(), rng.gen(), rng.gen(), 1.0);

        if is_struct {
            let inner_room_id = *next_room_id;
            *next_room_id += 1;

            // Tardis Effect: Inner room is much larger
            // Make height taller for effect
            let inner_w = size.x * ROOM_SCALE_FACTOR;
            let inner_h = size.y * ROOM_SCALE_FACTOR * 2.0;
            let inner_d = size.z * ROOM_SCALE_FACTOR;
            let inner_size = Vec3::new(inner_w, inner_h, inner_d);

            let mut inner_room = Room::new(inner_room_id, inner_size, color);

            // Recurse
            generate_recursive(world, &mut inner_room, next_room_id, depth + 1, rng);
            world.add_room(inner_room);

            let block = Block {
                pos,
                size,
                color,
                target_room_id: Some(inner_room_id),
                portal_face_normal: Vec3::Z, // Default to Z face for now
            };
            current_room.add_block(block);
        } else {
            // Primitive Value
            let block = Block {
                pos,
                size,
                color,
                target_room_id: None,
                portal_face_normal: Vec3::ZERO,
            };
            current_room.add_block(block);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation() {
        let world = generate_heap();
        assert!(world.rooms.len() > 0);

        // Check if root room has blocks
        // The root room is the last added, so it should be at the end or searched by ID
        let root = world.get_room(0).unwrap();
        assert!(root.blocks.len() > 0);
    }
}
