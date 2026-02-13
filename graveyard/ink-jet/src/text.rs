use rusttype::{point, Font, Scale};

pub fn rasterize_text(
    font_data: &[u8],
    text: &str,
    font_size: f32,
    grid_width: usize,
    grid_height: usize,
) -> Vec<bool> {
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

    let mut grid = vec![false; grid_width * grid_height];
    let scale = Scale::uniform(font_size);
    let _ = font.v_metrics(scale);

    // First pass layout at 0,0
    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();
    if glyphs.is_empty() {
        return grid;
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for glyph in &glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            min_x = min_x.min(bb.min.x);
            max_x = max_x.max(bb.max.x);
            min_y = min_y.min(bb.min.y);
            max_y = max_y.max(bb.max.y);
        }
    }

    if min_x == i32::MAX {
        return grid; // All spaces?
    }

    let text_width = max_x - min_x;
    let text_height = max_y - min_y;

    let start_x = (grid_width as f32 - text_width as f32) / 2.0 - min_x as f32;
    let start_y = (grid_height as f32 - text_height as f32) / 2.0 - min_y as f32;

    let offset = point(start_x, start_y);
    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

    for glyph in glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                if v > 0.5 {
                    let gx = bb.min.x + x as i32;
                    let gy = bb.min.y + y as i32;

                    if gx >= 0 && gx < grid_width as i32 && gy >= 0 && gy < grid_height as i32 {
                        let idx = gx as usize + gy as usize * grid_width;
                        grid[idx] = true;
                    }
                }
            });
        }
    }

    grid
}
