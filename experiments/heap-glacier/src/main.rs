use heap_glacier::simulation::HeapTerrain;
use macroquad::prelude::*;

#[macroquad::main("Heap Glacier")]
async fn main() {
    let width = 64;
    let height = 64;
    let mut terrain = HeapTerrain::new(width, height);

    let mut position = vec3(32.0, 40.0, 32.0);
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = -0.5;

    let mut auto_mode = 0;

    loop {
        // --- Input ---
        if is_key_down(KeyCode::W) {
            position += vec3(yaw.sin(), 0.0, yaw.cos()) * 0.5;
        }
        if is_key_down(KeyCode::S) {
            position -= vec3(yaw.sin(), 0.0, yaw.cos()) * 0.5;
        }
        if is_key_down(KeyCode::A) {
            position -= vec3(yaw.cos(), 0.0, -yaw.sin()) * 0.5;
        }
        if is_key_down(KeyCode::D) {
            position += vec3(yaw.cos(), 0.0, -yaw.sin()) * 0.5;
        }
        if is_key_down(KeyCode::Space) {
            position.y += 0.5;
        }
        if is_key_down(KeyCode::LeftShift) {
            position.y -= 0.5;
        }

        if is_key_down(KeyCode::Left) {
            yaw -= 0.05;
        }
        if is_key_down(KeyCode::Right) {
            yaw += 0.05;
        }
        if is_key_down(KeyCode::Up) {
            pitch += 0.05;
        }
        if is_key_down(KeyCode::Down) {
            pitch -= 0.05;
        }

        if is_key_pressed(KeyCode::Key1) {
            auto_mode = 1;
        }
        if is_key_pressed(KeyCode::Key2) {
            auto_mode = 2;
        }
        if is_key_pressed(KeyCode::Key3) {
            auto_mode = 3;
        }
        if is_key_pressed(KeyCode::Key0) {
            auto_mode = 0;
        }

        // --- Simulation ---
        match auto_mode {
            1 => {
                // Web Server
                for _ in 0..10 {
                    let x = rand::gen_range(0, width);
                    let y = rand::gen_range(0, height);
                    terrain.allocate(x, y, 0.5);
                }
                for _ in 0..8 {
                    let x = rand::gen_range(0, width);
                    let y = rand::gen_range(0, height);
                    terrain.deallocate(x, y, 0.5);
                }
            }
            2 => {
                // Leak
                for _ in 0..20 {
                    let x = rand::gen_range(0, width);
                    let y = rand::gen_range(0, height);
                    terrain.allocate(x, y, 1.0);
                }
            }
            3 => {
                // GC
                for _ in 0..50 {
                    let x = rand::gen_range(0, width);
                    let y = rand::gen_range(0, height);
                    terrain.deallocate(x, y, 2.0);
                }
            }
            _ => {}
        }

        terrain.tick();

        // --- Rendering ---
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position,
            up: vec3(0.0, 1.0, 0.0),
            target: position
                + vec3(
                    yaw.sin() * pitch.cos(),
                    pitch.sin(),
                    yaw.cos() * pitch.cos(),
                ),
            ..Default::default()
        });

        // Build Vertex Buffers
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut idx = 0;

        for y in 0..height - 1 {
            for x in 0..width - 1 {
                let h00 = terrain.get_bedrock(x, y);
                let h10 = terrain.get_bedrock(x + 1, y);
                let h01 = terrain.get_bedrock(x, y + 1);
                let h11 = terrain.get_bedrock(x + 1, y + 1);

                let i00 = terrain.get_ice(x, y);
                let i10 = terrain.get_ice(x + 1, y);
                let i01 = terrain.get_ice(x, y + 1);
                let i11 = terrain.get_ice(x + 1, y + 1);

                let w00 = terrain.get_water(x, y);
                let w10 = terrain.get_water(x + 1, y);
                let w01 = terrain.get_water(x, y + 1);
                let w11 = terrain.get_water(x + 1, y + 1);

                // Bedrock Layer
                vertices.push(vertex(x as f32, h00, y as f32, BROWN));
                vertices.push(vertex((x + 1) as f32, h10, y as f32, BROWN));
                vertices.push(vertex((x + 1) as f32, h11, (y + 1) as f32, BROWN));
                vertices.push(vertex(x as f32, h01, (y + 1) as f32, BROWN));

                indices.push(idx);
                indices.push(idx + 1);
                indices.push(idx + 2);
                indices.push(idx);
                indices.push(idx + 2);
                indices.push(idx + 3);
                idx += 4;

                // Ice Layer
                if i00 > 0.1 || i10 > 0.1 || i01 > 0.1 || i11 > 0.1 {
                    let color = Color::new(0.8, 1.0, 1.0, 0.8);
                    vertices.push(vertex(x as f32, h00 + i00, y as f32, color));
                    vertices.push(vertex((x + 1) as f32, h10 + i10, y as f32, color));
                    vertices.push(vertex((x + 1) as f32, h11 + i11, (y + 1) as f32, color));
                    vertices.push(vertex(x as f32, h01 + i01, (y + 1) as f32, color));

                    indices.push(idx);
                    indices.push(idx + 1);
                    indices.push(idx + 2);
                    indices.push(idx);
                    indices.push(idx + 2);
                    indices.push(idx + 3);
                    idx += 4;
                }

                // Water Layer
                if w00 > 0.01 || w10 > 0.01 || w01 > 0.01 || w11 > 0.01 {
                    let color = Color::new(0.0, 0.4, 1.0, 0.6);
                    let b00 = h00 + i00;
                    let b10 = h10 + i10;
                    let b01 = h01 + i01;
                    let b11 = h11 + i11;

                    vertices.push(vertex(x as f32, b00 + w00, y as f32, color));
                    vertices.push(vertex((x + 1) as f32, b10 + w10, y as f32, color));
                    vertices.push(vertex((x + 1) as f32, b11 + w11, (y + 1) as f32, color));
                    vertices.push(vertex(x as f32, b01 + w01, (y + 1) as f32, color));

                    indices.push(idx);
                    indices.push(idx + 1);
                    indices.push(idx + 2);
                    indices.push(idx);
                    indices.push(idx + 2);
                    indices.push(idx + 3);
                    idx += 4;
                }
            }
        }

        let mesh = Mesh {
            vertices,
            indices,
            texture: None,
        };
        draw_mesh(&mesh);

        set_default_camera();
        draw_text("Heap Glacier ⚛️🪨", 20.0, 30.0, 30.0, BLACK);
        draw_text(
            format!("Mode: {} (0:None, 1:Web, 2:Leak, 3:GC)", auto_mode).as_str(),
            20.0,
            60.0,
            20.0,
            BLACK,
        );
        draw_text("WASD+Arrows to move/look", 20.0, 80.0, 20.0, DARKGRAY);

        next_frame().await
    }
}

fn vertex(x: f32, y: f32, z: f32, color: Color) -> Vertex {
    Vertex {
        position: vec3(x, y, z),
        uv: vec2(0.0, 0.0),
        color: color.into(),
        normal: vec4(0.0, 1.0, 0.0, 0.0),
    }
}
