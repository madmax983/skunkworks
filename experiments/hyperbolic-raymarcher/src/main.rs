use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, neighbor_transform_a, Point, TilingConsts};
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

#[macroquad::main("Hyperbolic Raymarcher")]
async fn main() {
    let tiling = TilingConsts::new_4_5();

    // Player State
    // Position relative to current tile center
    let mut player_pos = Point::new(0.0, 0.0);
    // Path from root (0=Right, 1=Up, 2=Left, 3=Down)
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
            ],
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

        if is_key_down(KeyCode::W) { move_vec.im += speed; }
        if is_key_down(KeyCode::S) { move_vec.im -= speed; }
        if is_key_down(KeyCode::A) { move_vec.re -= speed; }
        if is_key_down(KeyCode::D) { move_vec.re += speed; }

        if move_vec.norm() > 0.0 {
            // Apply movement in hyperbolic space
            // P_new = mobius_add(P_old, move_vec)
            // Wait, move_vec is in the tangent space of the origin (Euclidean approximation for small steps).
            // mobius_add(z, a) translates z by a.
            // If we are at P, and want to move by `move_vec`, we should apply:
            // P_new = mobius_add(P, move_vec)
            // This works because `mobius_add` effectively moves the coordinate system.
            player_pos = mobius_add(player_pos, move_vec);
        }

        // Re-centering Logic
        // If player is closer to a neighbor than to current center (0), move to that neighbor.
        // Neighbors are at `neighbor_transform_a(k, &tiling)`.

        let mut best_neighbor = None;
        let mut min_dist_sq = player_pos.norm_sqr();

        for k in 0..4 {
            let nk = neighbor_transform_a(k, &tiling);
            // Transform player_pos to neighbor frame:
            // P_local = mobius_sub(player_pos, nk)
            let p_local = mobius_sub(player_pos, nk);
            let d_sq = p_local.norm_sqr();

            if d_sq < min_dist_sq {
                min_dist_sq = d_sq;
                best_neighbor = Some((k, p_local));
            }
        }

        if let Some((k, p_local)) = best_neighbor {
            // Transition to neighbor k
            player_pos = p_local;

            // Update path
            // In a tree, if we came from neighbor 'inv', and go back to 'inv', we pop.
            // Opposite of 0 is 2, 1 is 3.
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

        gl_use_material(&material);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);
        gl_use_default_material();

        // UI
        draw_text(&format!("Path Len: {}", current_path.len()), 10.0, 30.0, 20.0, WHITE);
        draw_text("WASD to Move", 10.0, 50.0, 20.0, WHITE);

        next_frame().await
    }
}
