use macroquad::prelude::*;
use glyph_terrain::{load_font, glyph_mesh, apply_terrain, GlyphMesh};

#[macroquad::main("Glyph Terrain")]
async fn main() {
    let font_bytes = include_bytes!("../assets/DejaVuSans.ttf");
    let font = load_font(font_bytes).expect("Failed to load font");

    let mut current_char = 'G';
    let mut mesh_mq = generate_mesh(&font, current_char);

    let mut cam_dist = 15.0;
    let mut cam_rot_x: f32 = 0.5;
    let mut cam_rot_y: f32 = 1.0;

    let mut last_mouse_pos = mouse_position();

    loop {
        if is_key_pressed(KeyCode::Space) {
             current_char = match current_char {
                 'Z' => 'A',
                 c => ((c as u8) + 1) as char,
             };
             mesh_mq = generate_mesh(&font, current_char);
        }

        if is_mouse_button_down(MouseButton::Left) {
            let m_pos = mouse_position();
            let dx = m_pos.0 - last_mouse_pos.0;
            let dy = m_pos.1 - last_mouse_pos.1;
            cam_rot_x += dx * 0.01;
            cam_rot_y = (cam_rot_y + dy * 0.01).clamp(0.1, 3.0);
        }
        last_mouse_pos = mouse_position();

        let scroll = mouse_wheel().1;
        cam_dist = (cam_dist - scroll).clamp(2.0, 50.0);

        clear_background(BLACK);

        set_camera(&Camera3D {
            position: vec3(
                cam_dist * cam_rot_x.cos() * cam_rot_y.sin(),
                cam_dist * cam_rot_y.cos(),
                cam_dist * cam_rot_x.sin() * cam_rot_y.sin()
            ),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_grid(20, 1.0, DARKGRAY, GRAY);

        draw_mesh(&mesh_mq);

        // Draw water plane
        draw_plane(vec3(0.0, -0.2, 0.0), vec2(20.0, 20.0), None, BLUE);

        set_default_camera();

        draw_text(&format!("Char: {}", current_char), 20.0, 30.0, 30.0, WHITE);
        draw_text("Drag to rotate, Scroll to zoom, Space to cycle", 20.0, 60.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}

fn generate_mesh(font: &impl ab_glyph::Font, c: char) -> Mesh {
    let mut gm = glyph_mesh(font, c);
    // Apply terrain: freq 0.5, amp 1.0
    apply_terrain(&mut gm, 42, 5.0, 1.0);

    to_mq_mesh(&gm)
}

fn to_mq_mesh(gm: &GlyphMesh) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::with_capacity(gm.vertices.len());

    // Create initial vertices with zero normal
    for v in &gm.vertices {
        vertices.push(Vertex {
            position: *v,
            uv: Vec2::ZERO,
            color: Color::new(0.2, 0.8, 0.3, 1.0).into(), // Greenish terrain color
            normal: Vec4::ZERO,
        });
    }

    // Calculate flat normals for triangles and accumulate to vertices
    for i in (0..gm.indices.len()).step_by(3) {
        let i0 = gm.indices[i] as usize;
        let i1 = gm.indices[i+1] as usize;
        let i2 = gm.indices[i+2] as usize;

        if i0 >= vertices.len() || i1 >= vertices.len() || i2 >= vertices.len() { continue; }

        let v0 = vertices[i0].position;
        let v1 = vertices[i1].position;
        let v2 = vertices[i2].position;

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(edge2).normalize_or_zero();

        // Convert to Vec4 for accumulation
        let n4 = vec4(normal.x, normal.y, normal.z, 0.0);

        // Accumulate normals
        vertices[i0].normal += n4;
        vertices[i1].normal += n4;
        vertices[i2].normal += n4;
    }

    // Normalize accumulated normals
    for v in &mut vertices {
        let n = v.normal;
        let len_sq = n.x*n.x + n.y*n.y + n.z*n.z;
        if len_sq > 0.0 {
            v.normal = n / len_sq.sqrt();
        } else {
            v.normal = vec4(0.0, 1.0, 0.0, 0.0); // Default up
        }

        // Apply color based on height (y)
        let height = v.position.y;
        let color: Color = if height < 0.2 {
             Color::new(0.8, 0.7, 0.5, 1.0) // Sand
        } else if height > 0.8 {
             WHITE // Snow
        } else {
             Color::new(0.1, 0.6, 0.1, 1.0) // Grass
        };
        v.color = color.into();
    }

    Mesh {
        vertices,
        indices: gm.indices.clone(),
        texture: None,
    }
}
