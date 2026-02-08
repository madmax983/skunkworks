use rusttype::{point, Font, Scale};

pub fn rasterize_text(text: &str, font_data: &[u8], width: usize, height: usize) -> Vec<bool> {
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

    // Initial guess for scale: fit height
    let scale_factor = 0.8;
    let initial_scale = Scale::uniform(height as f32 * scale_factor);

    // Layout to measure width
    let v_metrics = font.v_metrics(initial_scale);
    let glyphs: Vec<_> = font
        .layout(text, initial_scale, point(0.0, v_metrics.ascent))
        .collect();

    let width_px = if let Some(last) = glyphs.last() {
        last.position().x + last.unpositioned().h_metrics().advance_width
    } else {
        0.0
    };

    // Adjust scale to fit width if necessary
    let actual_scale = if width_px > width as f32 * 0.9 {
        initial_scale.x * (width as f32 * 0.9 / width_px)
    } else {
        initial_scale.x
    };

    let scale = Scale::uniform(actual_scale);
    let v_metrics = font.v_metrics(scale);

    // Calculate centered position
    // Center vertical:
    // Text height roughly ascent - descent
    // Baseline y should be such that (ascent + descent)/2 is at height/2
    // y = height/2 + (ascent + descent)/2 ?
    // Let's just use ascent/2 + height/2.
    // If we want the visual center of the text to be at height/2.
    // Visual center is roughly (ascent + descent) / 2 relative to baseline.
    // No, relative to baseline, top is -ascent, bottom is -descent (in Y-up).
    // In Y-down (screen), top is y - ascent, bottom is y - descent.
    // Center is y - (ascent + descent)/2.
    // We want Center = height/2.
    // y = height/2 + (ascent + descent)/2.
    let start_y = height as f32 / 2.0 + (v_metrics.ascent + v_metrics.descent) / 2.0;

    // Recalculate width with new scale
    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();
    let width_px = if let Some(last) = glyphs.last() {
        last.position().x + last.unpositioned().h_metrics().advance_width
    } else {
        0.0
    };
    let start_x = (width as f32 - width_px) / 2.0;

    let offset = point(start_x, start_y);
    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

    let mut grid = vec![false; width * height];

    for glyph in glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                // v is coverage [0, 1]
                let gx = x as i32 + bb.min.x;
                let gy = y as i32 + bb.min.y;

                if gx >= 0 && gx < width as i32 && gy >= 0 && gy < height as i32 {
                    if v > 0.5 {
                        grid[(gy as usize) * width + (gx as usize)] = true;
                    }
                }
            });
        }
    }

    grid
}
