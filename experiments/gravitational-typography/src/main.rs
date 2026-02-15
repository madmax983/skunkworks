use macroquad::prelude::*;

mod fluid;
mod particles;
mod physics;
mod shader;

use fluid::{FluidSim, HEIGHT, WIDTH};
use particles::ParticleSystem;
use physics::{Body, G};

fn window_conf() -> Conf {
    Conf {
        window_title: "Gravitational Typography ⚛️📜".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sim = FluidSim::new();
    let mut particle_system = ParticleSystem::new();
    let mut bodies = Vec::new();

    // Initial Setup: One massive star in center
    bodies.push(Body {
        pos: vec2(0.0, 0.0), // Center of view (0,0) for rendering logic
        vel: vec2(0.0, 0.0),
        mass: 5000.0,
        radius: 30.0,
        color: YELLOW,
    });

    // Add some orbiting bodies
    bodies.push(Body {
        pos: vec2(200.0, 0.0),
        vel: vec2(0.0, (G * 5000.0 / 200.0).sqrt()),
        mass: 200.0,
        radius: 10.0,
        color: BLUE,
    });

    // Load Shader
    let frag_src = shader::get_fragment_shader();
    let vert_src = shader::get_vertex_shader();
    let lensing_material = load_material(
        ShaderSource::Glsl {
            vertex: vert_src,
            fragment: &frag_src,
        },
        MaterialParams {
            uniforms: shader::get_uniforms(),
            pipeline_params: PipelineParams {
                depth_write: false,
                depth_test: Comparison::Always,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();

    let mut target = render_target(screen_width() as u32, screen_height() as u32);
    target.texture.set_filter(FilterMode::Linear);

    let mut show_fluid = true;
    let mut lensing_strength = 0.5;

    loop {
        let sw = screen_width();
        let sh = screen_height();
        let cell_w = sw / WIDTH as f32;
        let cell_h = sh / HEIGHT as f32;

        // Resize render target if needed
        if sw != target.texture.width() || sh != target.texture.height() {
            target = render_target(sw as u32, sh as u32);
            target.texture.set_filter(FilterMode::Linear);
        }

        let dt = get_frame_time();

        // Input: Spawn Particles (Typing)
        while let Some(c) = get_char_pressed() {
            if !c.is_control() && c != '\n' && c != '\r' && c != '\u{8}' {
                let mpos = mouse_position();
                let gx = mpos.0 / cell_w;
                let gy = mpos.1 / cell_h;

                let color = macroquad::color::hsl_to_rgb(macroquad::rand::gen_range(0.0, 1.0), 0.8, 0.8);

                // Spawn particle
                particle_system.spawn(gx, gy, c, color);

                // Add fluid impulse
                sim.add_velocity(gx as usize, gy as usize, 1.0, 0.0);
            }
        }

        // Input: Controls
        if is_key_pressed(KeyCode::F) {
            show_fluid = !show_fluid;
        }
        if is_key_pressed(KeyCode::Up) {
            lensing_strength += 0.1;
        }
        if is_key_pressed(KeyCode::Down) {
            lensing_strength -= 0.1;
        }

        // Input: Mouse (Add Bodies)
        if is_mouse_button_pressed(MouseButton::Left) {
             if bodies.len() < shader::MAX_BODIES {
                let mpos = mouse_position();
                // Convert to Physics coords (Center = 0,0)
                let phys_pos = vec2(mpos.0 - sw / 2.0, mpos.1 - sh / 2.0);

                bodies.push(Body {
                    pos: phys_pos,
                    vel: vec2(macroquad::rand::gen_range(-2.0, 2.0), macroquad::rand::gen_range(-2.0, 2.0)),
                    mass: macroquad::rand::gen_range(500.0, 3000.0),
                    radius: macroquad::rand::gen_range(10.0, 25.0),
                    color: Color::new(
                        macroquad::rand::gen_range(0.5, 1.0),
                        macroquad::rand::gen_range(0.5, 1.0),
                        macroquad::rand::gen_range(0.5, 1.0),
                        1.0,
                    ),
                });
             }
        }

        // Physics Update
        // Use sub-stepping for stability
        let substeps = 4;
        let sdt = dt / substeps as f32;
        for _ in 0..substeps {
            physics::integrate(&mut bodies, sdt);
        }

        // Map Bodies to Grid Coords for Fluid
        let mut mapped_bodies = Vec::with_capacity(bodies.len());
        for b in &bodies {
            let screen_x = b.pos.x + sw / 2.0;
            let screen_y = b.pos.y + sh / 2.0;
            let grid_x = screen_x / cell_w;
            let grid_y = screen_y / cell_h;
            // Scale mass for fluid interaction.
            // Physics Mass ~1000-5000.
            // Grid r ~10-100.
            // F = G * M / r^2. If G=0.5, r=10, we want F ~ 0.01.
            // 0.5 * M_scaled / 100 = 0.01 => M_scaled = 2.0.
            // So factor should be around 2.0 / 5000.0 = 0.0004.
            mapped_bodies.push((grid_x, grid_y, b.mass * 0.0005));
        }

        // Fluid Update
        sim.step(&mapped_bodies);

        particle_system.update(&sim, dt);

        // Rendering

        // 1. Render to Texture
        set_camera(&Camera2D {
            render_target: Some(target.clone()),
            zoom: vec2(2.0 / sw, 2.0 / sh), // Standard 2D zoom
            target: vec2(sw / 2.0, sh / 2.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // Draw Fluid Background
        if show_fluid {
             // Draw flow lines or density
             for y in (0..HEIGHT).step_by(2) {
                for x in (0..WIDTH).step_by(2) {
                    let idx = y * WIDTH + x;
                    let rho = sim.density[idx];
                    let diff = (rho - 1.0).abs();
                    if diff > 0.05 {
                         let color = Color::new(0.2, 0.4, 1.0, diff * 2.0);
                         draw_rectangle(
                            x as f32 * cell_w,
                            y as f32 * cell_h,
                            cell_w * 2.0,
                            cell_h * 2.0,
                            color,
                        );
                    }
                }
             }
        }

        // Draw Particles
        let font_size = cell_h * 1.5;
        for p in particle_system.particles() {
            let alpha = 1.0 - (p.lifetime / p.max_lifetime).powf(2.0);
            let mut color = p.color;
            color.a = alpha;

            draw_text(
                &p.char.to_string(),
                p.position.x * cell_w,
                p.position.y * cell_h + font_size,
                font_size,
                color,
            );
        }

        set_default_camera();

        // 2. Render Texture to Screen with Lensing
        gl_use_material(&lensing_material);

        lensing_material.set_uniform("ViewSize", vec2(sw, sh));
        lensing_material.set_uniform("ViewCenter", vec2(0.0, 0.0));

        lensing_material.set_uniform("Count", bodies.len() as i32);
        lensing_material.set_uniform("LensingStrength", lensing_strength);

        for (i, body) in bodies.iter().enumerate() {
            lensing_material.set_uniform(&format!("Body{}_Pos", i), body.pos);
            lensing_material.set_uniform(&format!("Body{}_Mass", i), body.mass);
        }

        draw_texture_ex(
            &target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(sw, sh)),
                flip_y: true,
                ..Default::default()
            },
        );

        gl_use_default_material();

        // 3. Draw Bodies (Indicators)
        let offset_x = sw / 2.0;
        let offset_y = sh / 2.0;

        for body in &bodies {
            draw_circle(
                body.pos.x + offset_x,
                body.pos.y + offset_y,
                body.radius,
                body.color,
            );
        }

        // UI
        draw_text("Gravitational Typography", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Particles: {}", particle_system.count()),
            10.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );
         draw_text(
            "Type to add text. Click to add stars.",
            10.0,
            sh - 20.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
