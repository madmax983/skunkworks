use macroquad::prelude::*;
use std::f32::consts::PI;

mod dna;
mod entropy;
mod archaeologist;

use dna::{Creature, Limb};

#[macroquad::main("Entropy Beast")]
async fn main() {
    // Initial State
    let mut original = Creature::random();
    let mut raw_data = original.serialize();
    let mut entropy = 0.0f32;

    // UI State
    let mut show_hex = true;

    loop {
        // Input
        if is_key_pressed(KeyCode::R) {
            original = Creature::random();
            raw_data = original.serialize();
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            entropy = (entropy + 0.001).min(1.0);
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            entropy = (entropy - 0.001).max(0.0);
        }
        if is_key_pressed(KeyCode::H) {
            show_hex = !show_hex;
        }

        // Logic: Corrupt & Resurrect
        let mut corrupted_data = raw_data.clone();
        entropy::corrupt(&mut corrupted_data, entropy);

        let reconstructed = archaeologist::resurrect(&corrupted_data);

        // Render
        clear_background(BLACK);

        // Draw Original (Left)
        draw_text("ORIGINAL", 50.0, 50.0, 30.0, WHITE);
        draw_creature(&original, vec2(screen_width() * 0.25, screen_height() * 0.5), 0.0);

        // Draw Reconstructed (Right)
        let label = format!("RECONSTRUCTED (Entropy: {:.1}%)", entropy * 100.0);
        draw_text(&label, screen_width() * 0.55, 50.0, 30.0,
            if entropy > 0.5 { RED } else { GREEN });

        draw_creature(&reconstructed, vec2(screen_width() * 0.75, screen_height() * 0.5), 0.0);

        // Draw Hex Strip
        if show_hex {
            draw_hex_strip(&raw_data, &corrupted_data, screen_height() - 100.0);
        }

        draw_text("Controls: [Arrows/AD] Entropy | [R] Regenerate | [H] Toggle Hex",
            20.0, screen_height() - 20.0, 20.0, GRAY);

        next_frame().await
    }
}

fn draw_creature(creature: &Creature, pos: Vec2, base_angle: f32) {
    // Draw Head
    let head_color = Color::from_rgba(creature.head.color[0], creature.head.color[1], creature.head.color[2], 255);
    draw_circle(pos.x, pos.y, creature.head.size, head_color);

    // Draw Eyes
    let eye_angle_step = PI / (creature.head.eye_count as f32 + 1.0);
    for i in 0..creature.head.eye_count {
        let angle = base_angle - PI/2.0 + eye_angle_step * (i as f32 + 1.0);
        let eye_pos = pos + vec2(angle.cos(), angle.sin()) * (creature.head.size * 0.6);
        draw_circle(eye_pos.x, eye_pos.y, creature.head.size * 0.2, WHITE);
        draw_circle(eye_pos.x, eye_pos.y, creature.head.size * 0.1, BLACK);
    }

    // Draw Limbs
    let limb_angle_step = (2.0 * PI) / creature.limbs.len().max(1) as f32;
    for (i, limb) in creature.limbs.iter().enumerate() {
        let angle = base_angle + i as f32 * limb_angle_step;
        draw_limb(limb, pos, angle, 0);
    }
}

fn draw_limb(limb: &Limb, parent_pos: Vec2, angle: f32, depth: u8) {
    let end_pos = parent_pos + vec2(angle.cos(), angle.sin()) * limb.length;

    // Draw Segment
    draw_line(parent_pos.x, parent_pos.y, end_pos.x, end_pos.y, limb.thickness, GRAY);
    draw_circle(end_pos.x, end_pos.y, limb.thickness, DARKGRAY); // Joint

    // Recurse
    if !limb.children.is_empty() {
        let child_angle_spread = PI / 2.0; // 90 degrees spread
        let child_angle_step = child_angle_spread / (limb.children.len() as f32 + 1.0);
        let start_angle = angle - child_angle_spread / 2.0;

        for (i, child) in limb.children.iter().enumerate() {
            let child_angle = start_angle + child_angle_step * (i as f32 + 1.0);
            draw_limb(child, end_pos, child_angle, depth + 1);
        }
    }
}

fn draw_hex_strip(original: &[u8], corrupted: &[u8], y: f32) {
    let block_w = (screen_width() / (original.len() as f32).max(1.0)).max(2.0); // Ensure minimal width
    let h = 50.0;

    for (i, &byte) in original.iter().enumerate() {
        // If we are out of corrupted range, treat as byte drop (missing)
        let is_corrupted = if i < corrupted.len() {
             byte != corrupted[i]
        } else {
             true // Missing data is corruption
        };

        let color = if is_corrupted { RED } else { Color::new(0.0, 1.0, 0.0, 0.3) };

        draw_rectangle(i as f32 * block_w, y, block_w - 0.5, h, color);
    }
}
