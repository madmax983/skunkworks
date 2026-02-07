use macroquad::prelude::*;
use sim::SimulationParams;

mod sim;
mod audio;

const SIM_WIDTH: u32 = 512;
const SIM_HEIGHT: u32 = 512;

#[macroquad::main("Fertile Soil")]
async fn main() {
    let mut params = SimulationParams::default();

    // Initialize Audio
    let drone = audio::Drone::new();
    if drone.is_none() {
        println!("Audio initialization failed or disabled.");
    }

    // Load Shaders
    let reaction_frag = include_str!("../shaders/reaction.glsl");
    let terrain_frag = include_str!("../shaders/terrain.glsl");
    let vertex_shader = include_str!("../shaders/vertex.glsl");

    let reaction_mat = load_material(
        ShaderSource::Glsl {
            vertex: vertex_shader,
            fragment: reaction_frag,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("Texture", UniformType::Int1),
                UniformDesc::new("ScreenSize", UniformType::Float2),
                UniformDesc::new("feed", UniformType::Float1),
                UniformDesc::new("kill", UniformType::Float1),
                UniformDesc::new("diff_a", UniformType::Float1),
                UniformDesc::new("diff_b", UniformType::Float1),
                UniformDesc::new("dt", UniformType::Float1),
            ],
            ..Default::default()
        },
    ).expect("Failed to load reaction material");

    let terrain_mat = load_material(
        ShaderSource::Glsl {
            vertex: vertex_shader,
            fragment: terrain_frag,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("Texture", UniformType::Int1),
                UniformDesc::new("ScreenSize", UniformType::Float2),
                UniformDesc::new("Time", UniformType::Float1),
            ],
            ..Default::default()
        },
    ).expect("Failed to load terrain material");

    // Setup Textures
    let mut current_texture = render_target(SIM_WIDTH, SIM_HEIGHT);
    let mut next_texture = render_target(SIM_WIDTH, SIM_HEIGHT);

    // Initialize State
    let initial_bytes = sim::get_initial_state(SIM_WIDTH as usize, SIM_HEIGHT as usize);
    let initial_tex = Texture2D::from_rgba8(SIM_WIDTH as u16, SIM_HEIGHT as u16, &initial_bytes);

    // Draw initial state to current_texture
    {
        let cam = Camera2D {
            render_target: Some(current_texture.clone()),
            zoom: vec2(2.0 / SIM_WIDTH as f32, 2.0 / SIM_HEIGHT as f32), // Map pixels to -1..1
            target: vec2(SIM_WIDTH as f32 / 2.0, SIM_HEIGHT as f32 / 2.0),
            ..Default::default()
        };
        set_camera(&cam);

        draw_texture(&initial_tex, 0.0, 0.0, WHITE);

        set_default_camera();
    }

    loop {
        // Input Handling
        if is_key_down(KeyCode::Up) { params.feed += 0.0001; }
        if is_key_down(KeyCode::Down) { params.feed -= 0.0001; }
        if is_key_down(KeyCode::Right) { params.kill += 0.0001; }
        if is_key_down(KeyCode::Left) { params.kill -= 0.0001; }

        if is_key_pressed(KeyCode::R) {
             // Reset logic (reload texture)
             let cam = Camera2D {
                render_target: Some(current_texture.clone()),
                zoom: vec2(2.0 / SIM_WIDTH as f32, 2.0 / SIM_HEIGHT as f32),
                target: vec2(SIM_WIDTH as f32 / 2.0, SIM_HEIGHT as f32 / 2.0),
                ..Default::default()
            };
            set_camera(&cam);
            draw_texture(&initial_tex, 0.0, 0.0, WHITE);
            set_default_camera();
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            let tx = (mx / sw) * SIM_WIDTH as f32;
            let ty = (my / sh) * SIM_HEIGHT as f32;

            let cam = Camera2D {
                render_target: Some(current_texture.clone()),
                zoom: vec2(2.0 / SIM_WIDTH as f32, 2.0 / SIM_HEIGHT as f32),
                target: vec2(SIM_WIDTH as f32 / 2.0, SIM_HEIGHT as f32 / 2.0),
                ..Default::default()
            };
            set_camera(&cam);
            draw_circle(tx, ty, 5.0, GREEN);
            set_default_camera();
        }

        // Audio Update
        if let Some(d) = &drone {
            // Modulate pitch based on params
            let base = 200.0;
            let mod_f = (params.feed - 0.03) * 10000.0; // range 0.03..0.07 -> 0..400
            let mod_k = (params.kill - 0.05) * 5000.0;
            d.set_freq((base + mod_f + mod_k).max(50.0));
        }

        // --- Simulation Step ---

        gl_use_material(&reaction_mat);

        reaction_mat.set_uniform("ScreenSize", vec2(SIM_WIDTH as f32, SIM_HEIGHT as f32));
        reaction_mat.set_uniform("feed", params.feed);
        reaction_mat.set_uniform("kill", params.kill);
        reaction_mat.set_uniform("diff_a", params.diff_a);
        reaction_mat.set_uniform("diff_b", params.diff_b);
        reaction_mat.set_uniform("dt", params.dt);

        {
            let cam = Camera2D {
                render_target: Some(next_texture.clone()),
                zoom: vec2(2.0 / SIM_WIDTH as f32, 2.0 / SIM_HEIGHT as f32),
                target: vec2(SIM_WIDTH as f32 / 2.0, SIM_HEIGHT as f32 / 2.0),
                ..Default::default()
            };
            set_camera(&cam);
            draw_texture(&current_texture.texture, 0.0, 0.0, WHITE);
            set_default_camera();
        }

        gl_use_default_material();

        std::mem::swap(&mut current_texture, &mut next_texture);

        // --- Terrain Render Step ---

        clear_background(BLACK);

        gl_use_material(&terrain_mat);
        terrain_mat.set_uniform("ScreenSize", vec2(SIM_WIDTH as f32, SIM_HEIGHT as f32));
        terrain_mat.set_uniform("Time", get_time() as f32);

        draw_texture_ex(
            &current_texture.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        gl_use_default_material();

        draw_text("FERTILE SOIL: Genesis", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(&format!("F: {:.4} (Up/Down)", params.feed), 10.0, 70.0, 20.0, LIGHTGRAY);
        draw_text(&format!("K: {:.4} (L/R)", params.kill), 10.0, 90.0, 20.0, LIGHTGRAY);
        draw_text("Click to Plant, R to Reset", 10.0, 110.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
