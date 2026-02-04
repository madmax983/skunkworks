use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Portal {
    pub target_room_id: usize,
    pub rect: Rect,
    pub color: Color,
}

#[derive(Clone, Debug)]
pub struct Room {
    pub id: usize,
    pub rect: Rect,
    pub color: Color,
    pub portals: Vec<Portal>,
    pub label: String,
}

pub struct World {
    pub rooms: HashMap<usize, Room>,
    pub root_id: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            root_id: 0,
        }
    }

    pub fn mock_memory() -> Self {
        let mut world = Self::new();

        // Room 0: Root Stack Frame
        let root = Room {
            id: 0,
            rect: Rect::new(-100.0, -100.0, 200.0, 200.0),
            color: GRAY,
            portals: vec![
                Portal {
                    target_room_id: 1,
                    rect: Rect::new(-50.0, -50.0, 40.0, 40.0),
                    color: RED,
                },
                Portal {
                    target_room_id: 2,
                    rect: Rect::new(10.0, -50.0, 40.0, 40.0),
                    color: BLUE,
                },
            ],
            label: "Stack Frame (Root)".to_string(),
        };
        world.rooms.insert(0, root);

        // Room 1: Heap Object A
        let room_a = Room {
            id: 1,
            rect: Rect::new(-200.0, -200.0, 400.0, 400.0), // Bigger internally
            color: RED,
            portals: vec![Portal {
                target_room_id: 2, // Points to B
                rect: Rect::new(50.0, 50.0, 100.0, 100.0),
                color: BLUE,
            }],
            label: "Heap Object A".to_string(),
        };
        world.rooms.insert(1, room_a);

        // Room 2: Heap Object B (Cycle to A)
        let room_b = Room {
            id: 2,
            rect: Rect::new(-200.0, -200.0, 400.0, 400.0),
            color: BLUE,
            portals: vec![
                Portal {
                    target_room_id: 1, // Points back to A
                    rect: Rect::new(-150.0, -150.0, 100.0, 100.0),
                    color: RED,
                },
                Portal {
                    target_room_id: 0, // Points back to Stack (Root)
                    rect: Rect::new(50.0, 50.0, 50.0, 50.0),
                    color: DARKGRAY,
                },
            ],
            label: "Heap Object B".to_string(),
        };
        world.rooms.insert(2, room_b);

        world
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_memory_cycles() {
        let world = World::mock_memory();
        assert_eq!(world.rooms.len(), 3);

        let root = world.rooms.get(&0).unwrap();
        let target_id = root.portals[0].target_room_id;
        assert_eq!(target_id, 1);

        let room_a = world.rooms.get(&target_id).unwrap();
        let target_id_2 = room_a.portals[0].target_room_id;
        assert_eq!(target_id_2, 2);

        let room_b = world.rooms.get(&target_id_2).unwrap();
        let target_id_3 = room_b.portals[0].target_room_id;
        assert_eq!(target_id_3, 1); // Cycle back to A
    }
}
