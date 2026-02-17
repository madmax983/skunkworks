use macroquad::prelude::*;

const SHADER_VERTEX: &str = include_str!("shaders/vertex.glsl");
const SHADER_SIM: &str = include_str!("shaders/water_sim.glsl");
const SHADER_RENDER: &str = include_str!("shaders/water_render.glsl");

#[macroquad::main("Liquid Tactics")]
async fn main() {
    let width = 512;
    let height = 512;

    let water_a = render_target(width as u32, height as u32);
    water_a.texture.set_filter(FilterMode::Nearest);
    let water_b = render_target(width as u32, height as u32);
    water_b.texture.set_filter(FilterMode::Nearest);

    let terrain = render_target(width as u32, height as u32);
    terrain.texture.set_filter(FilterMode::Linear);

    // Initialize Terrain
    {
        let camera = Camera2D {
            render_target: Some(terrain.clone()),
            zoom: vec2(2.0 / width as f32, 2.0 / height as f32),
            target: vec2(width as f32 / 2.0, height as f32 / 2.0),
            ..Default::default()
        };
        set_camera(&camera);
        clear_background(Color::new(0.0, 0.0, 0.0, 1.0)); // Base terrain level 0

        // Draw some islands
        draw_circle(width as f32 * 0.5, height as f32 * 0.5, 120.0, Color::new(0.4, 0.0, 0.0, 1.0));
        draw_circle(width as f32 * 0.2, height as f32 * 0.3, 60.0, Color::new(0.3, 0.0, 0.0, 1.0));
        draw_circle(width as f32 * 0.8, height as f32 * 0.7, 80.0, Color::new(0.5, 0.0, 0.0, 1.0));

        set_default_camera();
    }

    let sim_mat = load_material(
        ShaderSource::Glsl {
            vertex: SHADER_VERTEX,
            fragment: SHADER_SIM,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("_ScreenSize", UniformType::Float2),
                UniformDesc::new("_Mouse", UniformType::Float4),
            ],
            textures: vec![
                "_Texture".to_string(),
                "_Terrain".to_string(),
            ],
            ..Default::default()
        },
    ).unwrap();

    let render_mat = load_material(
        ShaderSource::Glsl {
            vertex: SHADER_VERTEX,
            fragment: SHADER_RENDER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("_ScreenSize", UniformType::Float2),
                UniformDesc::new("_Time", UniformType::Float1),
            ],
            textures: vec![
                "_Texture".to_string(),
                "_Terrain".to_string(),
            ],
            ..Default::default()
        },
    ).unwrap();

    let mut current_water = water_a;
    let mut next_water = water_b;

    loop {
        // Handle input mapping
        let mouse_pos = mouse_position();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let scale = (screen_w / width as f32).min(screen_h / height as f32);
        let draw_w = width as f32 * scale;
        let draw_h = height as f32 * scale;
        let offset_x = (screen_w - draw_w) / 2.0;
        let offset_y = (screen_h - draw_h) / 2.0;

        // Mouse in texture space
        let mx = (mouse_pos.0 - offset_x) / scale;
        let my = (mouse_pos.1 - offset_y) / scale;

        // Check bounds
        let in_bounds = mx >= 0.0 && mx < width as f32 && my >= 0.0 && my < height as f32;

        // 1. Edit Terrain
        if in_bounds && (is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right)) {
            let camera = Camera2D {
                render_target: Some(terrain.clone()),
                zoom: vec2(2.0 / width as f32, 2.0 / height as f32),
                target: vec2(width as f32 / 2.0, height as f32 / 2.0),
                ..Default::default()
            };
            set_camera(&camera);

            // Invert drawing Y just in case
            let draw_y = height as f32 - my;

            if is_mouse_button_down(MouseButton::Left) {
                draw_circle(mx, draw_y, 15.0, Color::new(1.0, 0.0, 0.0, 1.0)); // Raise terrain
            } else {
                draw_circle(mx, draw_y, 15.0, BLACK); // Lower/Erase
            }
            set_default_camera();
        }

        // 2. Simulate
        {
            let camera = Camera2D {
                render_target: Some(next_water.clone()),
                zoom: vec2(2.0 / width as f32, 2.0 / height as f32),
                target: vec2(width as f32 / 2.0, height as f32 / 2.0),
                ..Default::default()
            };
            set_camera(&camera);

            gl_use_material(&sim_mat);
            sim_mat.set_uniform("_ScreenSize", vec2(width as f32, height as f32));

            let click_strength = if in_bounds && is_mouse_button_down(MouseButton::Middle) { 1.0 } else { 0.0 };
            // Pass mouse as pixel coords.
            sim_mat.set_uniform("_Mouse", vec4(mx, height as f32 - my, click_strength, 20.0));

            sim_mat.set_texture("_Texture", current_water.texture.clone());
            sim_mat.set_texture("_Terrain", terrain.texture.clone());

            draw_texture_ex(
                &current_water.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(width as f32, height as f32)),
                    flip_y: true,
                    ..Default::default()
                },
            );
            gl_use_default_material();
            set_default_camera();
        }

        // Swap
        let temp = current_water;
        current_water = next_water;
        next_water = temp;

        // 3. Render
        clear_background(BLACK);

        gl_use_material(&render_mat);
        render_mat.set_uniform("_ScreenSize", vec2(width as f32, height as f32));
        render_mat.set_uniform("_Time", get_time() as f32);

        render_mat.set_texture("_Texture", current_water.texture.clone());
        render_mat.set_texture("_Terrain", terrain.texture.clone());

        draw_texture_ex(
            &current_water.texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();

        // UI
        draw_text("Liquid Tactics", 10.0, 30.0, 30.0, WHITE);
        draw_text("L/R Click: Terrain | Middle: Rain", 10.0, 50.0, 20.0, WHITE);

        next_frame().await
    }
}
