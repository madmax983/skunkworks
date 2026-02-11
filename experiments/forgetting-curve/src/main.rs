use macroquad::prelude::*;

mod quadtree;
use quadtree::{Node, QuadTree};

#[macroquad::main("Forgetting Curve")]
async fn main() {
    // Generate the "Memory" (Texture with text)
    // We render the source code of this file itself if possible, or just some placeholder text.
    let w = screen_width();
    let h = screen_height();
    let render_target = render_target(w as u32, h as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    // Draw the "Memory" once
    {
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, w, h));
        camera.render_target = Some(render_target.clone());
        set_camera(&camera);

        clear_background(WHITE);

        let text = r#"
fn forget(memory: &mut Memory) {
    if time > recall_time {
        memory.resolution *= 0.5;
    }
}

// The Forgetting Curve
// R = e^(-t/S)
// Where R is retrievability, S is stability of memory, t is time.

struct Memory {
    content: Vec<u8>,
    clarity: f32,
}

impl Memory {
    fn recall(&mut self) {
        self.clarity = 1.0;
    }
}

// Hover to Recall.
// Time decays all.
"#;

        let mut y = 40.0;
        for line in text.lines() {
            draw_text(line, 40.0, y, 30.0, BLACK);
            y += 40.0;
        }

        // Draw some shapes too
        draw_circle(600.0, 200.0, 50.0, BLUE);
        draw_rectangle(500.0, 400.0, 100.0, 100.0, RED);

        set_default_camera();
    }

    // Get the image data from GPU to CPU
    let image = render_target.texture.get_texture_data();

    // Main Loop
    let start_time = get_time();

    loop {
        clear_background(BLACK);

        let time = get_time() - start_time;

        // Decay function: Threshold increases exponentially with time
        // Start small (high detail), grow large (low detail)
        // threshold 0.0 = perfect copy
        // threshold 1.0 = very blocky
        let decay_rate = 0.05;
        let base_threshold = (time * decay_rate).exp() - 1.0;
        // Clamp mostly for sanity, though exp grows forever
        let base_threshold = base_threshold.clamp(0.0, 2.0);

        let mouse_pos = Vec2::from(mouse_position());

        // Build the QuadTree for this frame
        let tree = QuadTree::from_image(&image, base_threshold as f32, Some((mouse_pos, 200.0)));

        // Draw
        draw_node(&tree.root);

        // Draw UI
        draw_text(&format!("Time: {:.1}s", time), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Entropy: {:.3}", base_threshold), 10.0, 40.0, 20.0, WHITE);

        next_frame().await
    }
}

fn draw_node(node: &Node) {
    match node {
        Node::Leaf { rect, color, .. } => {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, *color);
            // Draw grid lines if the rect is large enough to see them
            if rect.w > 4.0 {
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, Color::new(0.0, 0.0, 0.0, 0.2));
            }
        },
        Node::Branch { children, .. } => {
            for child in children.iter() {
                draw_node(child);
            }
        }
    }
}
