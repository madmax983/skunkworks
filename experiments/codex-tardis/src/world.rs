use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Block {
    pub pos: Vec3,
    pub size: Vec3,
    pub color: Color,
    pub target_room_id: Option<usize>, // If Some, this is a portal
    #[allow(dead_code)]
    pub portal_face_normal: Vec3,      // Which face is the portal
}

// Splice: Added texture field
#[derive(Clone, Debug)]
pub struct Room {
    pub id: usize,
    pub size: Vec3,
    pub blocks: Vec<Block>,
    #[allow(dead_code)]
    pub background_color: Color,
    pub texture: Option<Texture2D>, // The Codex StarMap
}

impl Room {
    pub fn new(id: usize, size: Vec3, color: Color) -> Self {
        Self {
            id,
            size,
            blocks: Vec::new(),
            background_color: color,
            texture: None,
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

    #[allow(dead_code)]
    pub fn get_room_mut(&mut self, id: usize) -> Option<&mut Room> {
        self.rooms.iter_mut().find(|r| r.id == id)
    }
}
