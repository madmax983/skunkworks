use macroquad::prelude::*;
use life::LifeGame;

mod life;

#[macroquad::main("Projective Life")]
async fn main() {
    let width = 128;
    let height = 128;
    let mut game = LifeGame::new(width, height);
    game.randomize(macroquad::rand::rand() as u64);

    // Texture buffer
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    // Texture to hold the state
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    // Generate Mesh
    let mut mesh = generate_roman_surface(64, 64);
    mesh.texture = Some(texture.clone());

    let mut cam_dist = 3.0;
    let mut cam_angle_x: f32 = 1.0;
    let mut cam_angle_y: f32 = 0.5;

    let mut last_mouse_pos = mouse_position();
    let mut last_update = 0.0;
    let update_interval = 0.05;

    // Use a simpler material
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            pipeline_params: PipelineParams {
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                ..Default::default()
            },
            ..Default::default()
        },
    ).unwrap();

    loop {
        // Update Game
        if get_time() - last_update > update_interval {
            game.step();
            last_update = get_time();

            // Update Texture
            for y in 0..height {
                for x in 0..width {
                    if game.grid[y * width + x] == 1 {
                         image.set_pixel(x as u32, y as u32, WHITE);
                    } else {
                         image.set_pixel(x as u32, y as u32, BLACK);
                    }
                }
            }
            texture.update(&image);
        }

        // Camera Control
        let mouse_pos = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            let delta_x = mouse_pos.0 - last_mouse_pos.0;
            let delta_y = mouse_pos.1 - last_mouse_pos.1;
            cam_angle_x -= delta_x * 0.01;
            cam_angle_y += delta_y * 0.01;
        }
        last_mouse_pos = mouse_pos;

        cam_dist -= mouse_wheel().1 * 0.1;
        cam_dist = cam_dist.clamp(1.0, 10.0);

        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        // 3D Setup
        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
            cam_dist * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        gl_use_material(&material);
        draw_mesh(&mesh);
        gl_use_default_material();

        set_default_camera();

        draw_text("Projective Life", 10.0, 30.0, 30.0, WHITE);
        draw_text("Left Click + Drag to Rotate", 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("Scroll to Zoom", 10.0, 70.0, 20.0, LIGHTGRAY);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 90.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}

fn generate_roman_surface(slices: usize, stacks: usize) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=slices {
        let u = (i as f32 / slices as f32) * std::f32::consts::PI;
        for j in 0..=stacks {
            let v = (j as f32 / stacks as f32) * std::f32::consts::PI;

            // Roman Surface
            let x = 0.5 * (2.0 * u).sin() * v.sin().powi(2);
            let y = 0.5 * u.sin() * (2.0 * v).cos();
            let z = 0.5 * u.cos() * (2.0 * v).sin();

            vertices.push(Vertex {
                position: vec3(x, y, z),
                uv: vec2(i as f32 / slices as f32, j as f32 / stacks as f32),
                color: [255, 255, 255, 255],
                normal: vec4(0., 0., 0., 0.),
            });
        }
    }

    for i in 0..slices {
        for j in 0..stacks {
            let next_i = i + 1;
            let next_j = j + 1;

            let stride = stacks + 1;
            let a = (i * stride + j) as u16;
            let b = (next_i * stride + j) as u16;
            let c = (next_i * stride + next_j) as u16;
            let d = (i * stride + next_j) as u16;

            indices.push(a);
            indices.push(b);
            indices.push(d);

            indices.push(b);
            indices.push(c);
            indices.push(d);
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

const VERTEX_SHADER: &str = r#"
    #version 100
    attribute vec3 position;
    attribute vec2 texcoord;
    attribute vec4 color0;

    varying lowp vec2 uv;
    varying lowp vec4 color;

    uniform mat4 Model;
    uniform mat4 Projection;

    void main() {
        gl_Position = Projection * Model * vec4(position, 1);
        color = color0;
        uv = texcoord;
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    #version 100
    varying lowp vec2 uv;
    varying lowp vec4 color;

    uniform sampler2D Texture;

    void main() {
        gl_FragColor = color * texture2D(Texture, uv);
    }
"#;
