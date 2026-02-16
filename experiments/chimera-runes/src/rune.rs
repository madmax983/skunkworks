use image::{Rgba, RgbaImage};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub fn generate(hash: &[u8], width: u32, height: u32) -> RgbaImage {
    let mut img = RgbaImage::new(width, height);
    let mut rng = ChaCha8Rng::from_seed(hash_to_seed(hash));

    // 1. Background
    fill_background(&mut img, &mut rng);

    // 2. Draw Rune
    let cx = (width / 2) as i32;
    let cy = (height / 2) as i32;
    let max_radius = (width.min(height) as f32 * 0.45) as i32;

    draw_glyph(&mut img, &mut rng, cx, cy, max_radius, 0);

    img
}

fn hash_to_seed(hash: &[u8]) -> [u8; 32] {
    let mut seed = [0u8; 32];
    for (i, &b) in hash.iter().enumerate() {
        seed[i % 32] ^= b;
    }
    seed
}

fn fill_background(img: &mut RgbaImage, rng: &mut ChaCha8Rng) {
    let w = img.width();
    let h = img.height();

    // Dark noise
    for p in img.pixels_mut() {
        let n: u8 = rng.gen_range(0..20);
        *p = Rgba([n, n, n, 255]);
    }

    // Stars
    let num_stars = (w * h) / 500;
    for _ in 0..num_stars {
        let x = rng.gen_range(0..w);
        let y = rng.gen_range(0..h);
        let b = rng.gen_range(100..255);
        img.put_pixel(x, y, Rgba([b, b, b, 255]));
    }

    // Nebula patches (faint blobs)
    let num_blobs = rng.gen_range(3..8);
    for _ in 0..num_blobs {
        let x = rng.gen_range(0..w) as i32;
        let y = rng.gen_range(0..h) as i32;
        let r = rng.gen_range(20..100);
        let color = random_neon(rng);
        draw_filled_circle_additive(img, x, y, r, color, 0.05);
    }
}

fn draw_glyph(img: &mut RgbaImage, rng: &mut ChaCha8Rng, cx: i32, cy: i32, radius: i32, depth: u32) {
    if radius < 5 || depth > 4 {
        return;
    }

    let color = random_neon(rng);

    // Draw main circle
    draw_circle(img, cx, cy, radius, color);

    // Maybe draw a filled node at center
    if rng.gen_bool(0.3) {
        draw_filled_circle(img, cx, cy, radius / 5, color);
    }

    // Spawn sub-circles on perimeter
    let num_children = rng.gen_range(3..=6);
    let angle_offset = rng.gen_range(0.0..6.28);

    for i in 0..num_children {
        let angle = angle_offset + (i as f32 * 6.28 / num_children as f32);
        let nx = cx + (radius as f32 * angle.cos()) as i32;
        let ny = cy + (radius as f32 * angle.sin()) as i32;

        // Draw connection
        draw_line(img, cx, cy, nx, ny, color);

        // Recurse
        let next_radius = radius / 2; // Decay
        draw_glyph(img, rng, nx, ny, next_radius, depth + 1);
    }
}

fn random_neon(rng: &mut ChaCha8Rng) -> Rgba<u8> {
    let colors = [
        Rgba([0, 255, 255, 255]),   // Cyan
        Rgba([255, 0, 255, 255]),   // Magenta
        Rgba([57, 255, 20, 255]),   // Neon Green
        Rgba([0, 128, 255, 255]),   // Electric Blue
        Rgba([255, 255, 0, 255]),   // Yellow
        Rgba([255, 100, 0, 255]),   // Orange
    ];
    colors[rng.gen_range(0..colors.len())]
}

// --- Primitives ---

fn blend_additive(c1: Rgba<u8>, c2: Rgba<u8>, alpha: f32) -> Rgba<u8> {
    let r = (c1[0] as f32 + c2[0] as f32 * alpha).min(255.0) as u8;
    let g = (c1[1] as f32 + c2[1] as f32 * alpha).min(255.0) as u8;
    let b = (c1[2] as f32 + c2[2] as f32 * alpha).min(255.0) as u8;
    Rgba([r, g, b, 255])
}

fn put_pixel_safe(img: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
        img.put_pixel(x as u32, y as u32, color);
    }
}

fn put_pixel_additive(img: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>, alpha: f32) {
    if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
        let existing = *img.get_pixel(x as u32, y as u32);
        let blended = blend_additive(existing, color, alpha);
        img.put_pixel(x as u32, y as u32, blended);
    }
}

fn draw_line(img: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgba<u8>) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x = x0;
    let mut y = y0;

    loop {
        put_pixel_safe(img, x, y, color);
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn draw_circle(img: &mut RgbaImage, cx: i32, cy: i32, radius: i32, color: Rgba<u8>) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        put_pixel_safe(img, cx + x, cy + y, color);
        put_pixel_safe(img, cx + y, cy + x, color);
        put_pixel_safe(img, cx - y, cy + x, color);
        put_pixel_safe(img, cx - x, cy + y, color);
        put_pixel_safe(img, cx - x, cy - y, color);
        put_pixel_safe(img, cx - y, cy - x, color);
        put_pixel_safe(img, cx + y, cy - x, color);
        put_pixel_safe(img, cx + x, cy - y, color);

        if err <= 0 {
            y += 1;
            err += 2 * y + 1;
        }
        if err > 0 {
            x -= 1;
            err -= 2 * x + 1;
        }
    }
}

fn draw_filled_circle(img: &mut RgbaImage, cx: i32, cy: i32, radius: i32, color: Rgba<u8>) {
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x*x + y*y <= radius*radius {
                put_pixel_safe(img, cx + x, cy + y, color);
            }
        }
    }
}

fn draw_filled_circle_additive(img: &mut RgbaImage, cx: i32, cy: i32, radius: i32, color: Rgba<u8>, alpha: f32) {
    for y in -radius..=radius {
        for x in -radius..=radius {
            let dist_sq = x*x + y*y;
            if dist_sq <= radius*radius {
                // Fade out at edges
                let dist = (dist_sq as f32).sqrt();
                let fade = 1.0 - (dist / radius as f32);
                put_pixel_additive(img, cx + x, cy + y, color, alpha * fade);
            }
        }
    }
}
