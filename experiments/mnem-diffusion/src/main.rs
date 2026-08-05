//! # Entropy Morphogenesis 🍄
//!
//! **Genesis Experiment**: `mnem-diffusion`
//!
//! A biological simulation visualizing codebase entropy and decay using a reaction-diffusion model.
//!
//! ## Concept
//! This experiment merges the concept of code rot (`mnem-rot`) with chemical reaction-diffusion (`gray-scott`). It visualizes "Entropy Morphogenesis"—where healthy code diffuses a nutrient chemical ('U') while rotting code diffuses a kill chemical ('V'). This creates a visual representation of how codebase decay might spread and "infect" healthy areas if left unchecked.
//!
//! ## The Simulation
//! *   **The Substrate**: A 2D reaction-diffusion grid (`Gray-Scott`).
//! *   **The Code Nodes**: Simulated parts of a codebase that slowly rot over time (entropy increase).
//! *   **The Reaction**:
//!     *   **Healthy Code (Cyan)** injects 'U' (Feed chemical, visualized as blue/black).
//!     *   **Rotting Code (Magenta)** injects 'V' (Kill chemical, visualized as red/purple).
//! *   **The Result**: Code decay physically bleeds and diffuses across the space, forming complex, organic Turing patterns.
//!
//! ## Interactivity
//! *   **Left Click**: "Heal" nearby code nodes, resetting their entropy to zero and stopping the spread of decay.
//!
//! ## Tech Stack
//! *   **Rust**
//! *   **Macroquad** for visualization (WASM-ready).
//! *   **Gray-Scott** reaction-diffusion from `crates/gray-scott`.
//!
//! ## Running
//! ```bash
//! cargo run -p mnem-diffusion
//! ```
//!
use ::rand::Rng;
use gray_scott::GrayScott;
use macroquad::prelude::*;

// 🧬 Splice: Cross mnem-rot × gray-scott
// Lineage:
// - From mnem-rot: Graph node structures that have an 'entropy' value which decays and represents codebase rot.
// - From gray-scott: A Reaction-Diffusion grid.
// - Novel Trait: Entropy Morphogenesis. High-entropy rotting nodes inject 'V' (kill) chemical, while healthy nodes inject 'U' (feed) chemical.

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 200;

#[derive(Clone)]
struct Node {
    x: f32,
    y: f32,
    entropy: f32,    // 0.0 = healthy, 1.0 = full rot
    decay_rate: f32, // how fast it rots
}

#[macroquad::main("mnem-diffusion: Entropy Morphogenesis")]
async fn main() {
    let mut grid = GrayScott::new(GRID_WIDTH, GRID_HEIGHT);
    grid.diff_u = 1.0;
    grid.diff_v = 0.5;

    // Tweak config for interesting mushroom-like spreading of V
    let feed_rate = 0.055;
    let kill_rate = 0.062;

    let mut nodes: Vec<Node> = Vec::new();
    let mut rng = ::rand::thread_rng();

    // Spawn initial codebase graph nodes
    for _ in 0..15 {
        nodes.push(Node {
            x: rng.gen_range(20.0..(GRID_WIDTH as f32 - 20.0)),
            y: rng.gen_range(20.0..(GRID_HEIGHT as f32 - 20.0)),
            entropy: rng.gen_range(0.0..1.0),
            decay_rate: rng.gen_range(0.0001..0.001),
        });
    }

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        // Step the physics multiple times per frame for faster diffusion
        for _ in 0..10 {
            // Apply entropy injection
            for node in &mut nodes {
                // Rot over time
                node.entropy = (node.entropy + node.decay_rate).clamp(0.0, 1.0);

                let nx = node.x as usize;
                let ny = node.y as usize;

                if nx < GRID_WIDTH && ny < GRID_HEIGHT {
                    // Mnem-rot trait: High entropy injects V (decay), low entropy injects U (health/feed)
                    let v_injection = node.entropy * 0.1;
                    let u_injection = (1.0 - node.entropy) * 0.1;

                    // Inject into grid with a small radius
                    for dy in -2..=2 {
                        for dx in -2..=2 {
                            let cx = (nx as isize + dx) as usize;
                            let cy = (ny as isize + dy) as usize;
                            if cx < GRID_WIDTH && cy < GRID_HEIGHT {
                                let idx = cy * GRID_WIDTH + cx;
                                if let Some(u) = grid.u_mut().get_mut(idx) {
                                    *u = (*u + u_injection).clamp(0.0, 1.0);
                                }
                                if let Some(v) = grid.v_mut().get_mut(idx) {
                                    *v = (*v + v_injection).clamp(0.0, 1.0);
                                }
                            }
                        }
                    }
                }
            }

            grid.update(feed_rate, kill_rate, 1.0);
        }

        // Render
        let u_slice = grid.u();
        let v_slice = grid.v();

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = y * GRID_WIDTH + x;
                let u_val = u_slice[idx];
                let v_val = v_slice[idx];

                // Color mapping: U is blue (healthy code), V is red/purple (rotting code)
                let color = Color::new(v_val, 0.0, u_val * 0.5, 1.0);
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        texture.update(&image);

        clear_background(BLACK);

        // Draw texture scaled up
        let scale_x = screen_width() / GRID_WIDTH as f32;
        let scale_y = screen_height() / GRID_HEIGHT as f32;

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw the nodes
        for node in &nodes {
            let px = node.x * scale_x;
            let py = node.y * scale_y;

            // Healthy nodes are cyan, rotting nodes are magenta
            let node_color = Color::new(node.entropy, 1.0 - node.entropy, 1.0, 1.0);
            draw_circle(px, py, 4.0, node_color);
            draw_circle_lines(px, py, 6.0, 1.0, WHITE);
        }

        // Interactivity: "Heal" nodes by clicking near them
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let grid_mx = mx / scale_x;
            let grid_my = my / scale_y;

            for node in &mut nodes {
                let dx = node.x - grid_mx;
                let dy = node.y - grid_my;
                if dx * dx + dy * dy < 200.0 {
                    node.entropy = 0.0; // Reset rot
                }
            }
        }

        next_frame().await
    }
}

// Ensure tests pass if someone runs `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node {
            x: 10.0,
            y: 10.0,
            entropy: 0.5,
            decay_rate: 0.1,
        };
        assert_eq!(node.entropy, 0.5);
    }

    #[test]
    fn test_node_rot() {
        let mut node = Node {
            x: 10.0,
            y: 10.0,
            entropy: 0.5,
            decay_rate: 0.1,
        };
        node.entropy = (node.entropy + node.decay_rate).clamp(0.0, 1.0);
        assert!((node.entropy - 0.6).abs() < f32::EPSILON * 2.0); // Allow slight float imprecision
    }
}
