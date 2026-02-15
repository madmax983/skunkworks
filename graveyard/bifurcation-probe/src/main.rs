use bifurcation_probe::map::{ChaoticMap, LogisticMap};
use macroquad::prelude::*;
use rayon::prelude::*;

const WIDTH: usize = 1200;

fn draw_cobweb(r: f64, rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.0, 0.0, 0.0, 0.9),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, WHITE);

    // Draw Diagonal y = x
    draw_line(rect.x, rect.y + rect.h, rect.x + rect.w, rect.y, 1.0, GRAY);

    // Draw Function curve y = rx(1-x)
    let steps = 100;
    for i in 0..steps {
        let x1 = i as f64 / steps as f64;
        let x2 = (i + 1) as f64 / steps as f64;
        let y1 = r * x1 * (1.0 - x1);
        let y2 = r * x2 * (1.0 - x2);

        // Map to screen
        // x is [0, 1] -> [rect.x, rect.x + rect.w]
        // y is [0, 1] -> [rect.y + rect.h, rect.y] (flip Y)

        let sx1 = rect.x + (x1 as f32) * rect.w;
        let sy1 = rect.y + rect.h - (y1 as f32) * rect.h;
        let sx2 = rect.x + (x2 as f32) * rect.w;
        let sy2 = rect.y + rect.h - (y2 as f32) * rect.h;

        draw_line(sx1, sy1, sx2, sy2, 2.0, BLUE);
    }

    // Draw Orbit
    let mut x = 0.3; // Start at 0.3
    let mut curr_sx = rect.x + (x as f32) * rect.w;
    let mut curr_sy = rect.y + rect.h; // y=0 (start from x-axis)

    for _ in 0..100 {
        let next_x = r * x * (1.0 - x);

        // Vertical line to curve at (x, f(x))
        let target_sy = rect.y + rect.h - (next_x as f32) * rect.h;
        draw_line(curr_sx, curr_sy, curr_sx, target_sy, 1.0, YELLOW);

        // Horizontal line to diagonal at (f(x), f(x))
        let target_sx = rect.x + (next_x as f32) * rect.w;
        draw_line(curr_sx, target_sy, target_sx, target_sy, 1.0, YELLOW);

        x = next_x;
        curr_sx = target_sx;
        curr_sy = target_sy;
    }

    draw_text(
        &format!("Cobweb (r={:.5})", r),
        rect.x + 5.,
        rect.y + 20.,
        20.,
        WHITE,
    );
}
const HEIGHT: usize = 800;

fn map_value_to_pixel(val: f64, min_val: f64, max_val: f64, range: usize) -> Option<usize> {
    if val < min_val || val > max_val {
        return None;
    }
    let norm = (val - min_val) / (max_val - min_val);
    let idx = (norm * range as f64) as usize;
    if idx >= range {
        Some(range - 1)
    } else {
        Some(idx)
    }
}

#[macroquad::main("Bifurcation Probe")]
async fn main() {
    let mut image_data = vec![0u8; WIDTH * HEIGHT * 4];
    let texture = Texture2D::from_image(&Image {
        width: WIDTH as u16,
        height: HEIGHT as u16,
        bytes: image_data.clone(),
    });

    let mut min_r: f64 = 2.4;
    let mut max_r: f64 = 4.0;
    let min_x: f64 = 0.0;
    let max_x: f64 = 1.0;

    let mut dragging = false;
    let mut last_mouse_pos = Vec2::ZERO;

    let mut dirty = true;

    loop {
        // --- Input Handling ---

        // Zoom (Mouse Wheel)
        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            let zoom_factor = if wheel_y > 0.0 { 0.9 } else { 1.1 };
            let mouse_x = mouse_position().0 as f64;
            let screen_w = screen_width() as f64;
            let r_at_mouse = min_r + (max_r - min_r) * (mouse_x / screen_w);

            // Center zoom on mouse
            min_r = r_at_mouse - (r_at_mouse - min_r) * zoom_factor;
            max_r = r_at_mouse + (max_r - r_at_mouse) * zoom_factor;

            dirty = true;
        }

        // Pan (Right Click Drag)
        if is_mouse_button_pressed(MouseButton::Right) {
            dragging = true;
            last_mouse_pos = mouse_position().into();
        }
        if is_mouse_button_released(MouseButton::Right) {
            dragging = false;
        }
        if dragging {
            let mouse_pos: Vec2 = mouse_position().into();
            let delta = mouse_pos - last_mouse_pos;
            last_mouse_pos = mouse_pos;

            let r_per_pixel = (max_r - min_r) / screen_width() as f64;
            let delta_r = -delta.x as f64 * r_per_pixel;

            min_r += delta_r;
            max_r += delta_r;

            dirty = true;
        }

        // --- Rendering ---

        if dirty {
            // Compute columns in parallel
            let columns: Vec<(usize, f64, Vec<u8>)> = (0..WIDTH)
                .into_par_iter()
                .map(|px| {
                    let r = min_r + (max_r - min_r) * (px as f64 / WIDTH as f64);
                    let map = LogisticMap::new(r);

                    // Calculate Lyapunov exponent for color
                    // Use fewer steps for performance? Or just re-use the orbit?
                    // We'll compute it separately or during iteration.

                    // 1. Transient
                    let mut x = 0.5;
                    for _ in 0..500 {
                        x = map.iterate(x);
                    }

                    // 2. Compute Lyapunov + Histogram
                    let mut counts = vec![0u32; HEIGHT];
                    let mut sum_log_deriv = 0.0;
                    let steps = 1000;

                    for _ in 0..steps {
                        let deriv = map.derivative(x).abs();
                        if deriv > 1e-9 {
                            sum_log_deriv += deriv.ln();
                        } else {
                            sum_log_deriv += -10.0; // clamp
                        }

                        if let Some(y_idx) = map_value_to_pixel(x, min_x, max_x, HEIGHT) {
                            counts[y_idx] += 1;
                        }
                        x = map.iterate(x);
                    }

                    let lambda = sum_log_deriv / steps as f64;

                    // 3. Render Column
                    let mut col_pixels = vec![0u8; HEIGHT * 4];

                    // Color based on Lyapunov
                    // Stable (< 0) -> Blue/Cyan
                    // Chaotic (> 0) -> Red/Orange
                    let base_color = if lambda < 0.0 {
                        let intensity = (-lambda).min(1.0);
                        Color::new(0.0, 0.5 + 0.5 * intensity as f32, 1.0, 1.0)
                    } else {
                        let intensity = (lambda).min(1.0);
                        Color::new(1.0, 1.0 - intensity as f32, 0.0, 1.0)
                    };

                    // Find max count to normalize brightness?
                    // Or just log scale.
                    let max_count = counts.iter().max().copied().unwrap_or(1).max(1);

                    for y in 0..HEIGHT {
                        let count = counts[HEIGHT - 1 - y]; // Flip Y for image (0 is top)
                        if count > 0 {
                            // Alpha/Brightness based on count
                            let alpha = (count as f32 / max_count as f32).sqrt();

                            col_pixels[y * 4 + 0] = (base_color.r * 255.0 * alpha) as u8;
                            col_pixels[y * 4 + 1] = (base_color.g * 255.0 * alpha) as u8;
                            col_pixels[y * 4 + 2] = (base_color.b * 255.0 * alpha) as u8;
                            col_pixels[y * 4 + 3] = 255;
                        } else {
                            // Background (Black)
                            col_pixels[y * 4 + 3] = 255; // Opaque black
                        }
                    }

                    (px, lambda, col_pixels)
                })
                .collect();

            // Reconstruct Image
            for (px, _lambda, col_pixels) in columns {
                for y in 0..HEIGHT {
                    let idx = (y * WIDTH + px) * 4;
                    image_data[idx] = col_pixels[y * 4];
                    image_data[idx + 1] = col_pixels[y * 4 + 1];
                    image_data[idx + 2] = col_pixels[y * 4 + 2];
                    image_data[idx + 3] = col_pixels[y * 4 + 3];
                }
            }

            texture.update(&Image {
                width: WIDTH as u16,
                height: HEIGHT as u16,
                bytes: image_data.clone(),
            });

            dirty = false;
        }

        clear_background(BLACK);

        // Scale texture to fit screen
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // --- Overlay ---
        // Mouse Hover Info
        let mouse_pos = mouse_position();
        let r_hover = min_r + (max_r - min_r) * (mouse_pos.0 as f64 / screen_width() as f64);

        draw_text(&format!("R: {:.5}", r_hover), 10., 30., 20., WHITE);
        draw_text(
            "Left Click: Cobweb Plot | Right Click: Pan | Scroll: Zoom",
            10.,
            screen_height() - 10.,
            20.,
            WHITE,
        );

        if is_mouse_button_down(MouseButton::Left) {
            let size = screen_height().min(screen_width()) * 0.4;
            let rect = Rect::new(screen_width() - size - 10., 10., size, size);
            draw_cobweb(r_hover, rect);
        }

        next_frame().await;
    }
}
