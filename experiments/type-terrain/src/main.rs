use macroquad::prelude::*;
use rusttype::Font;

mod heightmap;
use heightmap::HeightMap;

#[macroquad::main("Type Terrain")]
async fn main() {
    let font_bytes = include_bytes!("../assets/font.ttf");
    let font = Font::try_from_bytes(font_bytes as &[u8]).expect("Error constructing Font");

    let width = 100; // Reduced to fit u16 index limit
    let height = 100;
    let scale = 60.0;
    let text = "TERRAIN";

    let map = HeightMap::from_text(text, &font, scale, width, height);

    let mesh = generate_terrain_mesh(&map);

    let mut cam_pos = vec3(width as f32 / 2.0, 30.0, height as f32 / 2.0 - 50.0);
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5; // Look down slightly

    loop {
        clear_background(BLACK);

        // Camera controls
        let speed = 0.5;
        let rot_speed = 0.02;

        if is_key_down(KeyCode::W) {
            cam_pos.x += cam_yaw.sin() * speed;
            cam_pos.z += cam_yaw.cos() * speed;
        }
        if is_key_down(KeyCode::S) {
            cam_pos.x -= cam_yaw.sin() * speed;
            cam_pos.z -= cam_yaw.cos() * speed;
        }
        if is_key_down(KeyCode::A) {
            cam_pos.x -= cam_yaw.cos() * speed;
            cam_pos.z += cam_yaw.sin() * speed;
        }
        if is_key_down(KeyCode::D) {
            cam_pos.x += cam_yaw.cos() * speed;
            cam_pos.z -= cam_yaw.sin() * speed;
        }
        if is_key_down(KeyCode::Space) {
            cam_pos.y += speed;
        }
        if is_key_down(KeyCode::LeftShift) {
            cam_pos.y -= speed;
        }

        if is_key_down(KeyCode::Left) {
            cam_yaw += rot_speed;
        }
        if is_key_down(KeyCode::Right) {
            cam_yaw -= rot_speed;
        }
        if is_key_down(KeyCode::Up) {
            cam_pitch += rot_speed;
        }
        if is_key_down(KeyCode::Down) {
            cam_pitch -= rot_speed;
        }

        // Mouse look (optional, but nice)
        // let (mx, my) = mouse_position();
        // ... omitted for simplicity in headless env

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0.0, 1.0, 0.0),
            target: cam_pos + vec3(cam_yaw.sin(), -cam_pitch.sin(), cam_yaw.cos()),
            ..Default::default()
        });

        draw_grid(20, 1.0, DARKGRAY, GRAY);

        // Draw mesh with wireframe toggle? No, just draw mesh.
        draw_mesh(&mesh);

        set_default_camera();
        draw_text("WASD to move, Arrows to look, Space/Shift to fly", 10.0, 20.0, 30.0, WHITE);
        draw_text(format!("Pos: {:.1}, {:.1}, {:.1}", cam_pos.x, cam_pos.y, cam_pos.z).as_str(), 10.0, 50.0, 20.0, WHITE);

        next_frame().await
    }
}

fn generate_terrain_mesh(map: &HeightMap) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for y in 0..map.height - 1 {
        for x in 0..map.width - 1 {
            let h00 = map.data[y * map.width + x];
            let h10 = map.data[y * map.width + (x + 1)];
            let h01 = map.data[(y + 1) * map.width + x];
            let h11 = map.data[(y + 1) * map.width + (x + 1)];

            // Generate colors based on height
            let c00 = height_color(h00);
            let c10 = height_color(h10);
            let c01 = height_color(h01);
            let c11 = height_color(h11);

            // 4 vertices for this quad
            let base_idx = vertices.len() as u16;

            // Using flat shading (normals up for now)
            // To make it look "poly", we duplicate vertices.

            vertices.push(Vertex {
                position: vec3(x as f32, h00, y as f32),
                uv: vec2(0.0, 0.0),
                color: c00.into(),
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            });
            vertices.push(Vertex {
                position: vec3((x + 1) as f32, h10, y as f32),
                uv: vec2(1.0, 0.0),
                color: c10.into(),
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            });
            vertices.push(Vertex {
                position: vec3((x + 1) as f32, h11, (y + 1) as f32),
                uv: vec2(1.0, 1.0),
                color: c11.into(),
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            });
            vertices.push(Vertex {
                position: vec3(x as f32, h01, (y + 1) as f32),
                uv: vec2(0.0, 1.0),
                color: c01.into(),
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            });

            // Triangle 1
            indices.push(base_idx);
            indices.push(base_idx + 1);
            indices.push(base_idx + 2);

            // Triangle 2
            indices.push(base_idx);
            indices.push(base_idx + 2);
            indices.push(base_idx + 3);
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

fn height_color(h: f32) -> Color {
    if h < 3.0 {
        Color::new(0.2, 0.2, 0.8, 1.0) // Water/Low
    } else if h < 6.0 {
        Color::new(0.2, 0.8, 0.2, 1.0) // Grass
    } else if h < 15.0 {
        Color::new(0.5, 0.5, 0.5, 1.0) // Rock
    } else {
        Color::new(0.9, 0.9, 0.9, 1.0) // Snow
    }
}
