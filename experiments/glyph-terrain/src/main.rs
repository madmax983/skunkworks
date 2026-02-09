use glyph_terrain::{
    create_grid_mesh, generate_glyph_heightmap, load_font, update_mesh_heights, HeightMap,
};
use macroquad::prelude::*;

const GRID_SIZE: usize = 128;
const GRID_SCALE: f32 = 40.0;
const NOISE_SCALE: f64 = 0.2;
const NOISE_AMP: f32 = 2.0;
const GLYPH_AMP: f32 = 12.0;

#[macroquad::main("Glyph Terrain")]
async fn main() {
    let font_bytes = include_bytes!("../assets/DejaVuSans.ttf");
    let font = load_font(font_bytes).expect("Failed to load font");

    // Initialize state
    let mut current_char = 'A';
    let mut target_char = 'A';

    // Heightmaps
    let mut h1 = generate_glyph_heightmap(&font, current_char, GRID_SIZE, GRID_SIZE);
    let mut h2 = generate_glyph_heightmap(&font, target_char, GRID_SIZE, GRID_SIZE);

    // The active blend
    let mut blended_heightmap = HeightMap::new(GRID_SIZE, GRID_SIZE);
    let mut morph_t: f32 = 0.0;
    let mut morphing = false;

    // Mesh
    let mut terrain_mesh = create_grid_mesh(GRID_SIZE, GRID_SCALE);

    // Camera
    let mut cam_pos = vec3(0.0, 15.0, 25.0);
    let mut cam_yaw: f32 = -90.0f32.to_radians();
    let mut cam_pitch: f32 = -30.0f32.to_radians();

    let mut last_mouse_pos = mouse_position();
    let mut mouse_captured = false;

    // Noise offset for animation
    let mut time: f64 = 0.0;

    loop {
        let dt = get_frame_time().min(0.1);
        time += dt as f64;

        // --- Input Handling ---

        // Toggle mouse capture
        if is_key_pressed(KeyCode::Tab) {
            mouse_captured = !mouse_captured;
            show_mouse(!mouse_captured);
            if mouse_captured {
                last_mouse_pos = mouse_position();
            }
        }

        // Camera rotation
        if mouse_captured {
            let m_pos = mouse_position();
            let dx = m_pos.0 - last_mouse_pos.0;
            let dy = m_pos.1 - last_mouse_pos.1;

            cam_yaw += dx * 0.005;
            cam_pitch -= dy * 0.005;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);

            last_mouse_pos = m_pos;
        } else {
            // Keep last_mouse_pos updated to avoid jump on re-capture
            last_mouse_pos = mouse_position();
        }

        // Camera movement
        let forward = vec3(
            cam_yaw.cos() * cam_pitch.cos(),
            cam_pitch.sin(),
            cam_yaw.sin() * cam_pitch.cos(),
        )
        .normalize();
        let right = forward.cross(vec3(0.0, 1.0, 0.0)).normalize();
        let up = vec3(0.0, 1.0, 0.0);

        let speed = if is_key_down(KeyCode::LeftShift) {
            20.0
        } else {
            8.0
        };
        let mut move_dir = Vec3::ZERO;

        if is_key_down(KeyCode::W) {
            move_dir += forward;
        }
        if is_key_down(KeyCode::S) {
            move_dir -= forward;
        }
        if is_key_down(KeyCode::A) {
            move_dir -= right;
        }
        if is_key_down(KeyCode::D) {
            move_dir += right;
        }
        if is_key_down(KeyCode::Space) {
            move_dir += up;
        }
        if is_key_down(KeyCode::LeftControl) {
            move_dir -= up;
        }

        if move_dir.length_squared() > 0.0 {
            cam_pos += move_dir.normalize() * speed * dt;
        }

        // Trigger morph
        if (is_key_pressed(KeyCode::Enter) || is_mouse_button_pressed(MouseButton::Left))
            && !morphing
        {
            target_char = match current_char {
                'Z' => 'A',
                c => ((c as u8) + 1) as char,
            };

            h2 = generate_glyph_heightmap(&font, target_char, GRID_SIZE, GRID_SIZE);
            morphing = true;
            morph_t = 0.0;
        }

        // --- Logic ---

        // Update Morph
        if morphing {
            morph_t += dt * 0.5; // 2 seconds transition
            if morph_t >= 1.0 {
                morph_t = 0.0;
                morphing = false;
                current_char = target_char;
                std::mem::swap(&mut h1, &mut h2);
            }
        }

        // Blend Heights
        for i in 0..blended_heightmap.data.len() {
            let v1 = h1.data[i];
            let v2 = h2.data[i];
            let t = if morphing {
                // Smoothstep
                morph_t * morph_t * (3.0 - 2.0 * morph_t)
            } else {
                0.0
            };
            blended_heightmap.data[i] = v1 * (1.0 - t) + v2 * t;
        }

        // Update Mesh and Bake Colors
        update_mesh_heights(
            &mut terrain_mesh,
            &blended_heightmap,
            123 + (time as u32 / 10),
            NOISE_SCALE,
            NOISE_AMP,
            GLYPH_AMP,
        );

        // --- Render ---
        clear_background(SKYBLUE);

        set_camera(&Camera3D {
            position: cam_pos,
            target: cam_pos + forward,
            up,
            ..Default::default()
        });

        draw_mesh(&terrain_mesh);

        // Water plane
        draw_plane(
            vec3(0.0, 1.5, 0.0),
            vec2(GRID_SCALE, GRID_SCALE),
            None,
            Color::new(0.0, 0.4, 0.8, 0.5),
        );

        set_default_camera();

        // UI
        draw_rectangle(10.0, 10.0, 500.0, 120.0, Color::new(0.0, 0.0, 0.0, 0.5));
        draw_text(
            &format!("Current: {} | Target: {}", current_char, target_char),
            20.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            "WASD+Shift: Fly | Tab: Capture Mouse | Enter/Click: Morph",
            20.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Pos: {:.1}, {:.1}, {:.1}", cam_pos.x, cam_pos.y, cam_pos.z),
            20.0,
            90.0,
            20.0,
            LIGHTGRAY,
        );
        if morphing {
            draw_text(
                &format!("Morphing: {:.0}%", morph_t * 100.0),
                20.0,
                120.0,
                20.0,
                YELLOW,
            );
        }

        next_frame().await;
    }
}
