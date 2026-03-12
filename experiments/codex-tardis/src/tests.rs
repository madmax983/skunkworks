use crate::{
    check_teleport,
    world::{Block, Room, World},
};
use macroquad::prelude::*;

#[test]
fn test_teleport_invalid_room_id_no_panic() {
    let mut world = World::new();

    let mut room = Room::new(0, Vec3::new(10.0, 10.0, 10.0), RED);
    let block = Block {
        pos: Vec3::ZERO,
        size: Vec3::ONE,
        color: GREEN,
        target_room_id: Some(999), // Target room doesn't exist
        portal_face_normal: Vec3::Z,
    };
    room.add_block(block);
    world.add_room(room);

    // Player is at 0, 0, 0, inside the block
    world.player.current_room_id = 0;
    world.player.pos = Vec3::ZERO;

    check_teleport(&mut world);
}
