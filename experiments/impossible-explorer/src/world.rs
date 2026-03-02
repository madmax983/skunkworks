use macroquad::prelude::*;

#[derive(Clone)]
pub struct Portal {
    pub pos: Vec3,
    pub size: Vec2,
    pub target_path: std::path::PathBuf,
    pub loaded_room: Option<Box<Room>>, // Cached room
}

#[derive(Clone)]
pub struct Room {
    pub path: std::path::PathBuf,
    pub size: Vec3,
    pub color: Color,
    pub portals: Vec<Portal>,
}

impl Room {
    pub fn render_transformed(&self, pos: Vec3, rot: Quat) {
        // Floor
        draw_transformed_cube(
            vec3(0.0, -self.size.y / 2.0 - 0.5, 0.0),
            vec3(self.size.x, 1.0, self.size.z),
            GRAY,
            pos,
            rot,
        );
        // Ceiling
        draw_transformed_cube(
            vec3(0.0, self.size.y / 2.0 + 0.5, 0.0),
            vec3(self.size.x, 1.0, self.size.z),
            DARKGRAY,
            pos,
            rot,
        );

        let wall_thickness = 1.0;

        // Walls
        // Back
        draw_transformed_cube(
            vec3(0.0, 0.0, -self.size.z / 2.0 - wall_thickness / 2.0),
            vec3(self.size.x, self.size.y, wall_thickness),
            self.color,
            pos,
            rot,
        );
        // Front
        draw_transformed_cube(
            vec3(0.0, 0.0, self.size.z / 2.0 + wall_thickness / 2.0),
            vec3(self.size.x, self.size.y, wall_thickness),
            self.color,
            pos,
            rot,
        );
        // Left
        draw_transformed_cube(
            vec3(-self.size.x / 2.0 - wall_thickness / 2.0, 0.0, 0.0),
            vec3(wall_thickness, self.size.y, self.size.z),
            self.color,
            pos,
            rot,
        );
        // Right
        draw_transformed_cube(
            vec3(self.size.x / 2.0 + wall_thickness / 2.0, 0.0, 0.0),
            vec3(wall_thickness, self.size.y, self.size.z),
            self.color,
            pos,
            rot,
        );

        // Render Portal Frames (Wireframe only for now, manual transform tricky for wires)
        for portal in &self.portals {
            // Rotation of portal frame matches room rotation
            // We can draw lines manually
            // Or just draw small cubes at corners
            // For MVP, skip wires if rotation is used, or implement wires transform
            // Let's just draw a transformed cube for frame
            draw_transformed_cube(
                portal.pos,
                vec3(portal.size.x, portal.size.y, 0.2),
                BLACK,
                pos,
                rot,
            );
        }
    }
}

fn draw_transformed_cube(local_pos: Vec3, size: Vec3, color: Color, world_pos: Vec3, rot: Quat) {
    // Generate vertices for a cube centered at local_pos with size
    let half_size = size / 2.0;

    let corners = [
        vec3(-1.0, -1.0, -1.0),
        vec3(1.0, -1.0, -1.0),
        vec3(1.0, 1.0, -1.0),
        vec3(-1.0, 1.0, -1.0),
        vec3(-1.0, -1.0, 1.0),
        vec3(1.0, -1.0, 1.0),
        vec3(1.0, 1.0, 1.0),
        vec3(-1.0, 1.0, 1.0),
    ];

    let vertices: Vec<Vec3> = corners
        .iter()
        .map(|c| {
            let p = local_pos + *c * half_size;
            rot * p + world_pos
        })
        .collect();

    // Draw 6 faces as quads (2 triangles each)
    // Indices for triangles
    let indices = [
        0, 1, 2, 0, 2, 3, // Front (actually back in local coords, -z)
        4, 6, 5, 4, 7, 6, // Back (+z)
        4, 5, 1, 4, 1, 0, // Bottom (-y)
        3, 2, 6, 3, 6, 7, // Top (+y)
        4, 0, 3, 4, 3, 7, // Left (-x)
        1, 5, 6, 1, 6, 2, // Right (+x)
    ];

    // Use macroquad's draw_mesh logic?
    // It's easier to use `draw_triangle_3d` directly.
    // Wait, indices refer to vertices.

    for i in (0..indices.len()).step_by(3) {
        let v1 = vertices[indices[i]];
        let v2 = vertices[indices[i + 1]];
        let v3 = vertices[indices[i + 2]];

        // Use wireframe for "Tron" aesthetic and simplicity
        draw_line_3d(v1, v2, color);
        draw_line_3d(v2, v3, color);
        draw_line_3d(v3, v1, color);

        // Optional: Diagonal for triangulation visual
        // draw_line_3d(v1, v3, color);
    }
}
