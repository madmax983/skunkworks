use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Portal {
    pub id: usize,
    pub pos: Vec3,  // Center position relative to room center
    pub rot: Quat,  // Rotation of the portal plane
    pub size: Vec2, // Width, Height
    pub target_room: usize,
    pub target_portal: usize, // The ID of the portal in the target room
}

#[derive(Clone, Debug)]
pub struct Room {
    pub id: usize,
    pub pos: Vec3,  // World position origin (for rendering separation)
    pub size: Vec3, // Half-extents? No, let's say full dimensions (W, H, D)
    pub color: Color,
    pub portals: Vec<Portal>,
}

#[derive(Clone, Debug)]
pub struct MemoryGraph {
    pub rooms: Vec<Room>,
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self { rooms: Vec::new() }
    }

    pub fn add_room(&mut self, pos: Vec3, size: Vec3, color: Color) -> usize {
        let id = self.rooms.len();
        self.rooms.push(Room {
            id,
            pos,
            size,
            color,
            portals: Vec::new(),
        });
        id
    }

    // Connects two rooms with a bidirectional portal
    // p1_pos: position in room 1
    // p2_pos: position in room 2
    pub fn connect(
        &mut self,
        r1: usize,
        p1_pos: Vec3,
        p1_rot: Quat,
        p1_size: Vec2,
        r2: usize,
        p2_pos: Vec3,
        p2_rot: Quat,
        p2_size: Vec2,
    ) {
        let p1_id = self.rooms[r1].portals.len();
        let p2_id = self.rooms[r2].portals.len();

        self.rooms[r1].portals.push(Portal {
            id: p1_id,
            pos: p1_pos,
            rot: p1_rot,
            size: p1_size,
            target_room: r2,
            target_portal: p2_id,
        });

        self.rooms[r2].portals.push(Portal {
            id: p2_id,
            pos: p2_pos,
            rot: p2_rot,
            size: p2_size,
            target_room: r1,
            target_portal: p1_id,
        });
    }
}

pub fn generate_heap() -> MemoryGraph {
    let mut graph = MemoryGraph::new();

    // Layout strategy: Place rooms far apart on X axis
    // Room 0: Stack Frame (Small)
    // 10x10x10
    let r0 = graph.add_room(
        vec3(0.0, 0.0, 0.0),
        vec3(10.0, 8.0, 10.0),
        Color::new(0.2, 0.2, 0.3, 1.0),
    );

    // Room 1: Heap Allocation (Huge) - The "Tardis" effect
    // 50x30x50
    // But accessed via a small door.
    let r1 = graph.add_room(
        vec3(100.0, 0.0, 0.0),
        vec3(40.0, 20.0, 40.0),
        Color::new(0.4, 0.1, 0.1, 1.0),
    );

    // Connect R0 and R1
    // Door in R0 is small (1x2) on the North Wall (+Z)
    // Door in R1 is large? Or same size?
    // Usually portals are same physical size on the frame, but the room behind is big.
    // So both portals are 1.5x2.5
    graph.connect(
        r0,
        vec3(0.0, -2.5, 4.9), // Near floor, on +Z wall
        Quat::from_rotation_y(std::f32::consts::PI), // Normal -Z (Inwards)
        vec2(2.0, 3.0),
        r1,
        vec3(0.0, -8.5, -19.9), // Near floor (-10 is bottom), on -Z wall (-20 is back)
        Quat::from_rotation_y(0.0), // Normal +Z (Inwards)
        vec2(2.0, 3.0),
    );

    // Room 2: Linked List Node A
    let r2 = graph.add_room(
        vec3(200.0, 0.0, 0.0),
        vec3(15.0, 10.0, 15.0),
        Color::new(0.1, 0.4, 0.1, 1.0),
    );

    // Connect R1 -> R2 (Pointer)
    // Door in R1 (East Wall +X)
    graph.connect(
        r1,
        vec3(19.9, -5.0, 0.0),                               // +X wall
        Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2), // Facing +X? Normal -X
        vec2(2.0, 3.0),
        r2,
        vec3(-7.4, -3.5, 0.0),                              // -X wall
        Quat::from_rotation_y(std::f32::consts::FRAC_PI_2), // Facing -X
        vec2(2.0, 3.0),
    );

    // Room 3: Linked List Node B
    let r3 = graph.add_room(
        vec3(300.0, 0.0, 0.0),
        vec3(15.0, 10.0, 15.0),
        Color::new(0.1, 0.3, 0.5, 1.0),
    );

    // Connect R2 -> R3
    graph.connect(
        r2,
        vec3(7.4, -3.5, 0.0),
        Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
        vec2(2.0, 3.0),
        r3,
        vec3(-7.4, -3.5, 0.0),
        Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        vec2(2.0, 3.0),
    );

    // Cycle: R3 -> R1
    // Back door in R3 (+Z) leads to Side door in R1 (-X)
    graph.connect(
        r3,
        vec3(0.0, -3.5, 7.4),
        Quat::from_rotation_y(std::f32::consts::PI), // Normal -Z (Inwards)
        vec2(2.0, 3.0),
        r1,
        vec3(-19.9, -5.0, 5.0), // -X wall, offset
        Quat::from_rotation_y(std::f32::consts::FRAC_PI_2), // Facing -X
        vec2(2.0, 3.0),
    );

    graph
}
