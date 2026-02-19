use macroquad::prelude::*;

pub const TILE_WIDTH: f32 = 40.0;
pub const TILE_HEIGHT: f32 = 20.0;
pub const Z_SCALE: f32 = 20.0; // Visual height of one z-unit

pub fn iso_project(u: i32, v: i32, w: i32) -> Vec2 {
    iso_project_f32(u as f32, v as f32, w as f32)
}

pub fn iso_project_f32(u: f32, v: f32, w: f32) -> Vec2 {
    let x = (u - v) * TILE_WIDTH * 0.5;
    let y = (u + v) * TILE_HEIGHT * 0.5 - (w * Z_SCALE);
    vec2(x, y)
}

pub fn draw_iso_block(u: i32, v: i32, w: i32, color: Color) {
    let pos = iso_project(u, v, w);
    let x = pos.x;
    let y = pos.y;

    let hw = TILE_WIDTH * 0.5;
    let hh = TILE_HEIGHT * 0.5;

    // Top face (diamond)
    // Center at (x, y - Z_SCALE) relative to base?
    // No, iso_project returns the "base" center on the grid.
    // But a block has height. Let's say it extends UP from w.
    // So the top face is at w+1 visually? Or just w?
    // Let's say w is the floor. The block sits on w.
    // So top face is at y - Z_SCALE.

    let top_center = vec2(x, y - Z_SCALE);
    let top_p1 = vec2(x, top_center.y - hh);
    let top_p2 = vec2(x + hw, top_center.y);
    let top_p3 = vec2(x, top_center.y + hh);
    let top_p4 = vec2(x - hw, top_center.y);

    draw_triangle(top_p1, top_p2, top_p3, color);
    draw_triangle(top_p1, top_p3, top_p4, color);

    // Right face
    let r_p1 = top_p2;
    let r_p2 = vec2(x + hw, top_center.y + Z_SCALE); // Down
    let r_p3 = vec2(x, top_center.y + hh + Z_SCALE);
    let r_p4 = top_p3;

    let dark_col = Color::new(color.r * 0.6, color.g * 0.6, color.b * 0.6, 1.0);
    draw_triangle(r_p1, r_p2, r_p3, dark_col);
    draw_triangle(r_p1, r_p3, r_p4, dark_col);

    // Left face
    let l_p1 = top_p4;
    let l_p2 = top_p3;
    let l_p3 = vec2(x, top_center.y + hh + Z_SCALE);
    let l_p4 = vec2(x - hw, top_center.y + Z_SCALE);

    let med_col = Color::new(color.r * 0.8, color.g * 0.8, color.b * 0.8, 1.0);
    draw_triangle(l_p1, l_p2, l_p3, med_col);
    draw_triangle(l_p1, l_p3, l_p4, med_col);

    // Outline
    draw_line(top_p1.x, top_p1.y, top_p2.x, top_p2.y, 1.0, BLACK);
    draw_line(top_p2.x, top_p2.y, top_p3.x, top_p3.y, 1.0, BLACK);
    draw_line(top_p3.x, top_p3.y, top_p4.x, top_p4.y, 1.0, BLACK);
    draw_line(top_p4.x, top_p4.y, top_p1.x, top_p1.y, 1.0, BLACK);

    draw_line(r_p2.x, r_p2.y, r_p3.x, r_p3.y, 1.0, BLACK);
    draw_line(l_p4.x, l_p4.y, l_p3.x, l_p3.y, 1.0, BLACK);
    draw_line(top_p2.x, top_p2.y, r_p2.x, r_p2.y, 1.0, BLACK);
    draw_line(top_p3.x, top_p3.y, l_p3.x, l_p3.y, 1.0, BLACK);
    draw_line(top_p4.x, top_p4.y, l_p4.x, l_p4.y, 1.0, BLACK);
}

pub fn draw_stair_path(u1: i32, v1: i32, w1: i32, u2: i32, v2: i32, w2: i32, color: Color) {
    // Determine path in grid (Manhattan)
    // We want to go from (u1,v1) to (u2,v2)
    // Steps:
    let steps_u = (u2 - u1).abs();
    let steps_v = (v2 - v1).abs();
    let steps_total = steps_u + steps_v;

    if steps_total == 0 {
        return;
    }

    // We want to ascend/descend smoothly?
    // "Impossible Stair": Always ascend.
    // If w2 > w1, we climb (w2 - w1) over steps_total.
    // If w2 <= w1 (cycle/back-edge), we behave as if w2 is HIGHER.
    // How much higher? Enough to look like we kept climbing.
    // e.g. target_h = w1 + steps_total / 2.0?

    let dz = if w2 > w1 {
        w2 - w1
    } else {
        // Force ascent
        steps_total.max(1) as i32
    };

    let step_h = dz as f32 / steps_total as f32;

    let mut curr_u = u1;
    let mut curr_v = v1;
    let mut curr_w = w1 as f32;

    let du = (u2 - u1).signum();
    let dv = (v2 - v1).signum();

    // Draw segments
    for _ in 0..steps_u {
        draw_step_segment(
            curr_u,
            curr_v,
            curr_w,
            curr_u + du,
            curr_v,
            curr_w + step_h,
            color,
        );
        curr_u += du;
        curr_w += step_h;
    }

    for _ in 0..steps_v {
        draw_step_segment(
            curr_u,
            curr_v,
            curr_w,
            curr_u,
            curr_v + dv,
            curr_w + step_h,
            color,
        );
        curr_v += dv;
        curr_w += step_h;
    }
}

fn draw_step_segment(u1: i32, v1: i32, w1: f32, u2: i32, v2: i32, w2: f32, color: Color) {
    // Draw a small ramp or stairs between two grid cells
    // Since we want "Escher" look, maybe just a simple thick line or a slab?
    // Let's draw a slab.

    let start = iso_project_f32(u1 as f32, v1 as f32, w1);
    let end = iso_project_f32(u2 as f32, v2 as f32, w2);

    // We draw a line for now, to keep it simple and clean
    draw_line(start.x, start.y, end.x, end.y, 4.0, color);
    draw_circle(start.x, start.y, 2.0, BLACK);
}
