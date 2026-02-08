use macroquad::prelude::*;
mod heightmap;
use heightmap::generate_text_heightmap;

const MAP_WIDTH: u32 = 200;
const MAP_HEIGHT: u32 = 200;

fn window_conf() -> Conf {
    Conf {
        window_title: "Text Terra".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font_data = include_bytes!("../assets/DejaVuSans.ttf");
    let heightmap = generate_text_heightmap("GENESIS", font_data, MAP_WIDTH, MAP_HEIGHT);

    let mut mesh = Mesh {
        vertices: Vec::new(),
        indices: Vec::new(),
        texture: None,
    };

    // Generate Mesh
    // Grid centered at 0,0
    let offset_x = MAP_WIDTH as f32 / 2.0;
    let offset_z = MAP_HEIGHT as f32 / 2.0;

    // Scale for display
    let scale = 0.5;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let h = heightmap.get(x, y);

            // Color based on height
            // < 5.0: Water/Sand
            // 5.0 - 15.0: Grass
            // > 15.0: Rock/Snow (Text is high)
            let color = if h < 2.0 {
                BLUE
            } else if h < 5.0 {
                BEIGE
            } else if h < 15.0 {
                GREEN
            } else if h < 25.0 {
                DARKGRAY
            } else {
                WHITE
            };

            mesh.vertices.push(Vertex {
                position: vec3(
                    (x as f32 - offset_x) * scale,
                    h * 0.5, // Vertical scale
                    (y as f32 - offset_z) * scale,
                ),
                uv: vec2(x as f32 / MAP_WIDTH as f32, y as f32 / MAP_HEIGHT as f32),
                color: color.into(),
                // macroquad 0.4 Vertex normal is Vec4? Or maybe just not used correctly.
                // Let's assume Vec4 if compiler says so.
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            });
        }
    }

    // Generate Indices
    for y in 0..MAP_HEIGHT - 1 {
        for x in 0..MAP_WIDTH - 1 {
            let i = (y * MAP_WIDTH + x) as u16;
            let next_row = ((y + 1) * MAP_WIDTH + x) as u16;

            // Triangle 1
            mesh.indices.push(i);
            mesh.indices.push(next_row);
            mesh.indices.push(i + 1);

            // Triangle 2
            mesh.indices.push(i + 1);
            mesh.indices.push(next_row);
            mesh.indices.push(next_row + 1);
        }
    }

    // Camera state
    let mut cam_angle_x: f32 = 0.5;
    let mut cam_angle_y: f32 = 1.0;
    let mut cam_dist: f32 = 100.0;

    loop {
        clear_background(SKYBLUE);

        // Input
        if is_key_down(KeyCode::Left) {
            cam_angle_y -= 0.02;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y += 0.02;
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x = (cam_angle_x + 0.02).min(1.5);
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x = (cam_angle_x - 0.02).max(0.1);
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 1.0;
        }

        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_mesh(&mesh);

        draw_grid(20, 1.0, BLACK, GRAY);

        set_default_camera();

        draw_text("ARROWS to Rotate, W/S to Zoom", 10.0, 20.0, 30.0, BLACK);
        draw_text(
            "Terrain generated from text 'GENESIS'",
            10.0,
            50.0,
            20.0,
            DARKGRAY,
        );

        next_frame().await
    }
}
