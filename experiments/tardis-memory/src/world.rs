use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Block {
    pub pos: Vec3,
    pub size: Vec3,
    pub color: Color,
    pub target_room_id: Option<usize>, // If Some, this is a portal
    pub portal_face_normal: Vec3,      // Which face is the portal (usually +Z or -Z relative to block)
}

#[derive(Clone, Debug)]
pub struct Room {
    pub id: usize,
    pub size: Vec3, // Dimensions of the room
    pub blocks: Vec<Block>,
    pub background_color: Color,
}

impl Room {
    pub fn new(id: usize, size: Vec3, color: Color) -> Self {
        Self {
            id,
            size,
            blocks: Vec::new(),
            background_color: color,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

pub struct Player {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub current_room_id: usize,
}

impl Player {
    pub fn new(start_pos: Vec3, start_room: usize) -> Self {
        Self {
            pos: start_pos,
            yaw: -90.0,
            pitch: 0.0,
            current_room_id: start_room,
        }
    }
}

pub struct World {
    pub rooms: Vec<Room>,
    pub player: Player,
}

impl World {
    pub fn new() -> Self {
        Self {
            rooms: Vec::new(),
            player: Player::new(Vec3::new(0.0, 2.0, 0.0), 0),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    pub fn get_room(&self, id: usize) -> Option<&Room> {
        self.rooms.iter().find(|r| r.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_creation() {
        let mut world = World::new();
        let room = Room::new(0, Vec3::new(20.0, 10.0, 20.0), BLUE);
        world.add_room(room);

        assert_eq!(world.rooms.len(), 1);
        assert_eq!(world.get_room(0).unwrap().id, 0);
    }

    #[test]
    fn test_block_addition() {
        let mut room = Room::new(0, Vec3::new(10.0, 10.0, 10.0), RED);
        let block = Block {
            pos: Vec3::ZERO,
            size: Vec3::ONE,
            color: GREEN,
            target_room_id: None,
            portal_face_normal: Vec3::Z,
        };
        room.add_block(block);
        assert_eq!(room.blocks.len(), 1);
    }
}
