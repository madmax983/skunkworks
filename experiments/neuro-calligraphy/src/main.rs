mod creature;
mod font;
mod neuro;
mod physics;

use creature::ContourCreature;
use font::{load_glyph, Segment};
use macroquad::prelude::*;
use physics::PhysicsWorld;
use ttf_parser::Face;

fn resample_contour(contour: &[Vec2], spacing: f32) -> Vec<Vec2> {
    if contour.len() < 2 {
        return contour.to_vec();
    }

    let n = contour.len();
    // Calculate total length
    let mut total_len = 0.0;
    for i in 0..n {
        let p1 = contour[i];
        let p2 = contour[(i + 1) % n];
        total_len += (p2 - p1).length();
    }

    let num_points = (total_len / spacing).round().max(3.0) as usize;
    let actual_spacing = total_len / num_points as f32;

    let mut resampled = Vec::new();

    let mut current_pos = contour[0];
    let mut next_idx = 1;

    // We walk along the perimeter
    for _ in 0..num_points {
        resampled.push(current_pos);

        let mut dist_needed = actual_spacing;

        // Loop to find next point
        // Max iterations to prevent infinite loop
        for _ in 0..n*2 {
            if dist_needed <= 0.001 { break; }

            let p_next = contour[next_idx];
            let dist_to_next = (p_next - current_pos).length();

            if dist_to_next > dist_needed {
                let dir = (p_next - current_pos).normalize_or_zero();
                current_pos += dir * dist_needed;
                dist_needed = 0.0;
            } else {
                dist_needed -= dist_to_next;
                current_pos = p_next;
                next_idx = (next_idx + 1) % n;
            }
        }
    }

    resampled
}

fn quadratic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let mt = 1.0 - t;
    p0 * mt * mt + p1 * 2.0 * mt * t + p2 * t * t
}

fn cubic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let mt = 1.0 - t;
    p0 * mt * mt * mt + p1 * 3.0 * mt * mt * t + p2 * 3.0 * mt * t * t + p3 * t * t * t
}

#[macroquad::main("Neuro Calligraphy")]
async fn main() {
    let font_data = include_bytes!("../assets/font.ttf");
    let face = Face::parse(font_data, 0).expect("Failed to parse font");

    let text = "BIO-LOGIC";
    let scale = 0.25;
    let spacing = 15.0; // Point spacing for creatures
    let start_x = 50.0;
    let start_y = 350.0;

    let mut world = PhysicsWorld::new();
    world.bounds = Rect::new(0.0, 0.0, screen_width(), screen_height());
    world.gravity = vec2(0.0, 0.0);
    world.drag = 0.05;

    let mut creatures = Vec::new();

    let mut x_cursor = start_x;

    for c in text.chars() {
        if let Some(outline) = load_glyph(&face, c) {
             for contour in outline.contours {
                 let mut points = Vec::new();
                 points.push(vec2(contour.start.x, contour.start.y));

                 for segment in contour.segments {
                     match segment {
                         Segment::Line(p) => points.push(vec2(p.x, p.y)),
                         Segment::Quad(p1, p2) => {
                             let start = *points.last().unwrap();
                             let steps = 5;
                             for i in 1..=steps {
                                 let t = i as f32 / steps as f32;
                                 let p = quadratic_bezier(start, vec2(p1.x, p1.y), vec2(p2.x, p2.y), t);
                                 points.push(p);
                             }
                         },
                         Segment::Cubic(p1, p2, p3) => {
                             let start = *points.last().unwrap();
                             let steps = 5;
                             for i in 1..=steps {
                                 let t = i as f32 / steps as f32;
                                 let p = cubic_bezier(start, vec2(p1.x, p1.y), vec2(p2.x, p2.y), vec2(p3.x, p3.y), t);
                                 points.push(p);
                             }
                         }
                     }
                 }

                 // Transform to screen space
                 let world_points: Vec<Vec2> = points.iter().map(|p| {
                     vec2(x_cursor + p.x * scale, start_y - p.y * scale)
                 }).collect();

                 // Resample
                 let resampled = resample_contour(&world_points, spacing);

                 if resampled.len() > 5 {
                     creatures.push(ContourCreature::new(&mut world, &resampled, 8.0));
                 }
             }
             x_cursor += outline.advance_width * scale + 20.0;
        }
    }

    loop {
        // Handle Resize
        world.bounds = Rect::new(0.0, 0.0, screen_width(), screen_height());

        // Input
        if is_mouse_button_down(MouseButton::Left) {
            let m_pos = mouse_position();
            let m_vec = vec2(m_pos.0, m_pos.1);
            for p in &mut world.points {
                let d = p.pos - m_vec;
                let dist = d.length();
                if dist < 150.0 && dist > 1.0 {
                    p.apply_force(d.normalize() * 1000.0 / dist);
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
             // Reset? Or burst drive?
             for c in &mut creatures {
                 c.base_drive += 5.0;
             }
        }

        let dt = get_frame_time();

        // Update
        for c in &mut creatures {
            c.update(dt, &mut world);
            // Decay drive back to base
            if c.base_drive > 10.0 {
                c.base_drive *= 0.99;
            }
        }

        world.update(dt);

        // Draw
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        for c in &creatures {
            c.draw(&world);
        }

        draw_text("NEURO-CALLIGRAPHY", 20.0, 30.0, 30.0, WHITE);
        draw_text("Biological Typography Simulation", 20.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("Click to Disturb | Space to Excite", 20.0, screen_height() - 20.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
