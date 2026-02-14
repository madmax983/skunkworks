use crate::map::{MemoryGraph, Portal};
use macroquad::prelude::*;

pub struct PortalRenderer {
    pub targets: Vec<RenderTarget>,
}

impl PortalRenderer {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
        }
    }

    pub fn get_target(&mut self, index: usize, w: f32, h: f32) -> RenderTarget {
        while self.targets.len() <= index {
            let target = render_target(w as u32, h as u32);
            target.texture.set_filter(FilterMode::Linear);
            self.targets.push(target);
        }
        self.targets[index].clone()
    }
}

pub fn draw_room(graph: &MemoryGraph, room_id: usize) {
    let room = &graph.rooms[room_id];
    let half_size = room.size / 2.0;

    // Draw floor
    draw_cube(
        room.pos + vec3(0.0, -half_size.y - 0.1, 0.0),
        vec3(room.size.x, 0.2, room.size.z),
        None,
        room.color,
    );
    // Draw ceiling
    draw_cube(
        room.pos + vec3(0.0, half_size.y + 0.1, 0.0),
        vec3(room.size.x, 0.2, room.size.z),
        None,
        room.color,
    );

    // Walls
    // Back (-Z)
    draw_cube(
        room.pos + vec3(0.0, 0.0, -half_size.z - 0.1),
        vec3(room.size.x, room.size.y, 0.2),
        None,
        room.color,
    );
    // Front (+Z)
    draw_cube(
        room.pos + vec3(0.0, 0.0, half_size.z + 0.1),
        vec3(room.size.x, room.size.y, 0.2),
        None,
        room.color,
    );
    // Left (-X)
    draw_cube(
        room.pos + vec3(-half_size.x - 0.1, 0.0, 0.0),
        vec3(0.2, room.size.y, room.size.z),
        None,
        room.color,
    );
    // Right (+X)
    draw_cube(
        room.pos + vec3(half_size.x + 0.1, 0.0, 0.0),
        vec3(0.2, room.size.y, room.size.z),
        None,
        room.color,
    );

    // Draw Portals Frame
    for portal in &room.portals {
        let portal_world_pos = room.pos + portal.pos;
        // Simple wireframe for now
        draw_cube_wires(
            portal_world_pos,
            vec3(portal.size.x, portal.size.y, 0.2),
            BLACK,
        );
    }
}

pub fn render_portal_view(
    renderer: &mut PortalRenderer,
    graph: &MemoryGraph,
    src_room_id: usize,
    portal: &Portal,
    cam_pos: Vec3,
    cam_target: Vec3,
    cam_up: Vec3,
    depth: usize,
    target_index: usize,
    screen_size: Vec2,
) -> Option<Texture2D> {
    if depth == 0 {
        return None;
    }

    let dest_room = &graph.rooms[portal.target_room];
    let dest_portal = &dest_room.portals[portal.target_portal];
    let src_room = &graph.rooms[src_room_id];

    // Matrices
    let src_room_mat = Mat4::from_translation(src_room.pos);
    let src_portal_mat =
        src_room_mat * Mat4::from_translation(portal.pos) * Mat4::from_quat(portal.rot);

    let dest_room_mat = Mat4::from_translation(dest_room.pos);
    let dest_portal_mat =
        dest_room_mat * Mat4::from_translation(dest_portal.pos) * Mat4::from_quat(dest_portal.rot);

    // Flip rotation (180 deg Y)
    let flip = Mat4::from_rotation_y(std::f32::consts::PI);

    // Transform: T = Dest * Flip * Src^-1
    let transform = dest_portal_mat * flip * src_portal_mat.inverse();

    // Transform Camera
    let new_pos = transform.transform_point3(cam_pos);
    let new_target = transform.transform_point3(cam_target);
    // Transform UP vector (as direction, so w=0)
    let new_up = transform.transform_vector3(cam_up);

    let target = renderer.get_target(target_index, screen_size.x, screen_size.y);

    let cam = Camera3D {
        position: new_pos,
        target: new_target,
        up: new_up,
        render_target: Some(target.clone()),
        fovy: 45.0, // Default fov?
        ..Default::default()
    };

    set_camera(&cam);

    // Draw destination
    clear_background(dest_room.color);
    draw_room(graph, portal.target_room);

    // Recurse? (Not yet implemented, just 1 level)

    set_default_camera();

    Some(target.texture)
}
