use macroquad::prelude::*;
use poincare_disk::{Point, mobius_add, mobius_sub};

mod history;
mod layout;

use history::load_repo;
use layout::layout_graph;

#[macroquad::main("Hyperbolic Mycelium")]
async fn main() {
    let repo_path = "."; // Defaults to current directory
    let commits = load_repo(repo_path).unwrap_or_else(|e| {
        eprintln!("Failed to load repo: {}", e);
        Vec::new()
    });

    let layout = layout_graph(&commits);

    // Player position (center of screen in hyperbolic space)
    let mut player_pos = Point::new(0.0, 0.0);

    // Spores
    struct Spore {
        from: String,
        to: String,
        progress: f32,
    }
    let mut spores: Vec<Spore> = Vec::new();

    // Spawn some initial spores
    if !commits.is_empty() {
        // ...
    }

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Deep Blue-Black

        // Input
        let speed = 0.02;
        let mut move_vec = Point::new(0.0, 0.0);
        if is_key_down(KeyCode::W) { move_vec.im += speed; }
        if is_key_down(KeyCode::S) { move_vec.im -= speed; }
        if is_key_down(KeyCode::A) { move_vec.re -= speed; }
        if is_key_down(KeyCode::D) { move_vec.re += speed; }

        // Scale with zoom?

        if move_vec.norm() > 0.0 {
            // Move player: new_pos = player (+) move
            player_pos = mobius_add(player_pos, move_vec);
        }

        if is_key_pressed(KeyCode::Space) {
            // Spawn spores
             if !commits.is_empty() {
                // We want to spawn from random nodes
                let idx = rand::gen_range(0, commits.len());
                let start_node = &commits[idx];
                for parent in &start_node.parents {
                     spores.push(Spore { from: start_node.oid.clone(), to: parent.clone(), progress: 0.0 });
                }
             }
        }

        // Render
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let scale = screen_height().min(screen_width()) / 2.0 * 0.95;

        // Draw Boundary
        draw_circle_lines(center.x, center.y, scale, 2.0, Color::new(0.2, 0.2, 0.4, 1.0));

        // Draw Edges (Geodesics)
        for commit in &commits {
            if let Some(pos) = layout.get(&commit.oid) {
                // Transform to screen space (relative to player)
                // z_screen = mobius_sub(z_world, player_pos)
                let z_screen = mobius_sub(*pos, player_pos);

                if z_screen.norm_sqr() >= 1.0 { continue; } // Clipping

                // Draw edges to parents
                for parent_oid in &commit.parents {
                    if let Some(parent_pos) = layout.get(parent_oid) {
                         let parent_screen = mobius_sub(*parent_pos, player_pos);
                         if parent_screen.norm_sqr() >= 1.0 { continue; }

                         let p1 = vec2(
                             center.x + z_screen.re as f32 * scale,
                             center.y - z_screen.im as f32 * scale
                         );
                         let p2 = vec2(
                             center.x + parent_screen.re as f32 * scale,
                             center.y - parent_screen.im as f32 * scale
                         );

                         draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, Color::new(0.3, 0.5, 0.3, 0.3));
                    }
                }

                // Draw Node
                let screen_x = center.x + z_screen.re as f32 * scale;
                let screen_y = center.y - z_screen.im as f32 * scale;

                // Size depends on distance from center (perspective)
                // 1 - r^2 metric factor
                let r = z_screen.norm();
                let metric = 1.0 - r*r;
                let size = 3.0 * metric as f32; // Smaller at edge

                if size > 0.5 {
                    draw_circle(screen_x, screen_y, size, WHITE);
                }
            }
        }

        // Update and Draw Spores
        spores.retain_mut(|spore| {
            spore.progress += 0.01;
            if spore.progress >= 1.0 { return false; }

            if let (Some(from_pos), Some(to_pos)) = (layout.get(&spore.from), layout.get(&spore.to)) {
                 // Interpolate in screen space for simplicity (wrong but fast)
                 // Correct way: Interpolate in hyperbolic space, then transform.

                 // Interpolation in hyperbolic space?
                 // Geodesic(t).
                 // For now, linear interpolation of the complex numbers is "Chordal" motion.

                 let from_screen = mobius_sub(*from_pos, player_pos);
                 let to_screen = mobius_sub(*to_pos, player_pos);

                 let curr = from_screen + (to_screen - from_screen) * spore.progress as f64;

                 let screen_x = center.x + curr.re as f32 * scale;
                 let screen_y = center.y - curr.im as f32 * scale;

                 draw_circle(screen_x, screen_y, 2.0, YELLOW);
            }
            true
        });

        // UI
        draw_text("Hyperbolic Mycelium", 10.0, 30.0, 30.0, WHITE);
        draw_text(format!("Commits: {}", commits.len()).as_str(), 10.0, 50.0, 20.0, GRAY);
        draw_text("WASD to Pan. SPACE to Spore.", 10.0, 70.0, 20.0, GRAY);

        next_frame().await
    }
}
