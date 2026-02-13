
use macroquad::prelude::*;
use crate::heap::{Heap, Block};

const GOLDEN_RATIO_CONJUGATE: f32 = 0.618033988749895;

pub fn draw_heap(heap: &Heap, screen_w: f32, screen_h: f32) {
    let total_area = screen_w * screen_h;
    let cell_size = (total_area / heap.total_size as f32).sqrt();
    let cell_w = cell_size;
    let cell_h = cell_size;

    let cols = (screen_w / cell_w).floor() as usize;
    if cols == 0 { return; }

    for block in &heap.blocks {
        draw_block(block, cols, cell_w, cell_h);
    }
}

fn draw_block(block: &Block, cols: usize, cell_w: f32, cell_h: f32) {
    let mut current_idx = block.start;
    let end_idx = block.start + block.size;

    let color = if block.is_free {
        if block.lifetime > 0.0 {
            let h = (block.id as f32 * GOLDEN_RATIO_CONJUGATE) % 1.0;
            let mut c = hsv_to_rgb(h, 0.5, 0.5);
            c.a = block.lifetime;
            c
        } else {
            Color::new(0.1, 0.1, 0.1, 1.0)
        }
    } else {
        let h = (block.id as f32 * GOLDEN_RATIO_CONJUGATE) % 1.0;
        hsv_to_rgb(h, 0.7, 0.9)
    };

    while current_idx < end_idx {
        let col = current_idx % cols;
        let row = current_idx / cols;

        let available_in_row = cols - col;
        let remaining = end_idx - current_idx;
        let draw_count = available_in_row.min(remaining);

        let x = col as f32 * cell_w;
        let y = row as f32 * cell_h;
        let w = draw_count as f32 * cell_w;
        let h = cell_h;

        draw_rectangle(x, y, w, h, color);

        current_idx += draw_count;
    }
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (r, g, b) = match i as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => (v, p, q),
    };
    Color::new(r, g, b, 1.0)
}
