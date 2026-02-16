use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, neighbor_transform_a, Point, TilingConsts};
use rusttype::{Font, Scale};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod shader;

// Generate a deterministic float seed from a path
fn hash_path(path: &[u8]) -> f32 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    let h = hasher.finish();
    // Normalize to [0, 1]
    (h as f64 / u64::MAX as f64) as f32
}

fn generate_font_atlas(font_bytes: &[u8]) -> Texture2D {
    let font = Font::try_from_bytes(font_bytes).expect("Error constructing Font");

    let atlas_size = 512;
    let grid_cols = 10;
    let grid_rows = 10;
    let cell_size = atlas_size / grid_cols;

    // Create RGBA image buffer (black transparent)
    let mut pixels = vec![0u8; (atlas_size * atlas_size * 4) as usize];

    let scale = Scale::uniform(cell_size as f32 * 0.8); // 80% of cell size

    // Characters to render (Space to ~)
    let start_char = b' '; // 32

    for i in 0..(grid_cols * grid_rows) {
        let char_code = start_char + i as u8;
        if char_code > 126 { break; } // ASCII limit

        let c = char_code as char;
        let glyph = font.glyph(c).scaled(scale).positioned(rusttype::point(0.0, 0.0));

        // Calculate cell position
        let col = i % grid_cols;
        let row = i / grid_cols;

        let cell_x = col * cell_size;
        let cell_y = row * cell_size;

        // Center the glyph in the cell
        let bb = glyph.pixel_bounding_box().unwrap_or(rusttype::Rect { min: rusttype::Point { x: 0, y: 0 }, max: rusttype::Point { x: 0, y: 0 } });
        let glyph_w = bb.width();
        let glyph_h = bb.height();

        let offset_x = (cell_size as i32 - glyph_w) / 2;
        let offset_y = (cell_size as i32 - glyph_h) / 2; // Approximate centering

        // Draw glyph
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                // v is coverage [0.0, 1.0]
                let gx = cell_x as i32 + offset_x + x as i32 + bb.min.x;
                let gy = cell_y as i32 + offset_y + y as i32 + bb.min.y;

                if gx >= 0 && gx < atlas_size as i32 && gy >= 0 && gy < atlas_size as i32 {
                    let idx = ((gy as usize * atlas_size as usize) + gx as usize) * 4;
                    let val = (v * 255.0) as u8;

                    // Additive blending (max)
                    if val > pixels[idx] {
                        pixels[idx] = val;     // R
                        pixels[idx+1] = val;   // G
                        pixels[idx+2] = val;   // B
                        pixels[idx+3] = 255;   // A (Full Alpha if there is any pixel)
                    }
                }
            });
        }
    }

    let image = Image {
        width: atlas_size as u16,
        height: atlas_size as u16,
        bytes: pixels,
    };

    Texture2D::from_image(&image)
}

#[macroquad::main("Hyperbolic Library")]
async fn main() {
    // Load Font
    let possible_paths = [
        "assets/font.ttf",
        "experiments/type-terrain/assets/font.ttf",
        "../experiments/type-terrain/assets/font.ttf",
        "../../experiments/type-terrain/assets/font.ttf",
    ];

    let mut font_bytes = None;
    for path in possible_paths {
        if let Ok(bytes) = load_file(path).await {
            font_bytes = Some(bytes);
            println!("Loaded font from {}", path);
            break;
        }
    }

    let font_bytes = font_bytes.expect("Failed to load font.ttf. Run from appropriate directory.");
    let font_texture = generate_font_atlas(&font_bytes);
    font_texture.set_filter(FilterMode::Linear);

    let tiling = TilingConsts::new_4_5();

    // Player State
    let mut player_pos = Point::new(0.0, 0.0);
    let mut current_path: Vec<u8> = Vec::new();

    // Compile Shader
    let material = load_material(
        ShaderSource::Glsl {
            vertex: shader::VERTEX,
            fragment: shader::FRAGMENT,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float3),
                UniformDesc::new("iTime", UniformType::Float1),
                UniformDesc::new("u_player_pos", UniformType::Float2),
                UniformDesc::new("u_neighbor_offset", UniformType::Float1),
                UniformDesc::new("u_root_seed", UniformType::Float1),
                UniformDesc::new("u_grid_cols", UniformType::Float1),
                UniformDesc::new("u_grid_rows", UniformType::Float1),
            ],
            textures: vec!["u_font_texture".to_string()],
            ..Default::default()
        },
    )
    .unwrap();

    let start_time = get_time();

    loop {
        clear_background(BLACK);

        // Input Handling
        let speed = 0.02;
        let mut move_vec = Point::new(0.0, 0.0);

        if is_key_down(KeyCode::W) {
            move_vec.im += speed;
        }
        if is_key_down(KeyCode::S) {
            move_vec.im -= speed;
        }
        if is_key_down(KeyCode::A) {
            move_vec.re -= speed;
        }
        if is_key_down(KeyCode::D) {
            move_vec.re += speed;
        }

        if move_vec.norm() > 0.0 {
            player_pos = mobius_add(player_pos, move_vec);
        }

        // Re-centering Logic
        let mut best_neighbor = None;
        let mut min_dist_sq = player_pos.norm_sqr();

        for k in 0..4 {
            let nk = neighbor_transform_a(k, &tiling);
            let p_local = mobius_sub(player_pos, nk);
            let d_sq = p_local.norm_sqr();

            if d_sq < min_dist_sq {
                min_dist_sq = d_sq;
                best_neighbor = Some((k, p_local));
            }
        }

        if let Some((k, p_local)) = best_neighbor {
            player_pos = p_local;
            let opposite = (k + 2) % 4;
            let last = current_path.last().cloned();

            if let Some(l) = last {
                if l == opposite as u8 {
                    current_path.pop();
                } else {
                    current_path.push(k as u8);
                }
            } else {
                current_path.push(k as u8);
            }
        }

        // Prepare Uniforms
        let root_seed = hash_path(&current_path);

        material.set_uniform("iResolution", (screen_width(), screen_height(), 0.0));
        material.set_uniform("iTime", (get_time() - start_time) as f32);
        material.set_uniform("u_player_pos", (player_pos.re as f32, player_pos.im as f32));
        material.set_uniform("u_neighbor_offset", tiling.neighbor_offset as f32);
        material.set_uniform("u_root_seed", root_seed);
        material.set_uniform("u_grid_cols", 10.0f32);
        material.set_uniform("u_grid_rows", 10.0f32);

        material.set_texture("u_font_texture", font_texture.clone());

        gl_use_material(&material);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);
        gl_use_default_material();

        // UI
        draw_text(
            &format!("Depth: {}", current_path.len()),
            10.0,
            30.0,
            20.0,
            WHITE,
        );
        draw_text("WASD to Navigate the Library", 10.0, 50.0, 20.0, GRAY);

        // FPS
        draw_text(&format!("FPS: {}", get_fps()), screen_width() - 100.0, 30.0, 20.0, WHITE);

        next_frame().await
    }
}
