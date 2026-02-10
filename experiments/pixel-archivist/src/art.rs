use image::{DynamicImage, Rgb, RgbImage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};

pub fn generate_cover(data: &[u8]) -> DynamicImage {
    // 1. Calculate required size
    // 4 bytes header + data
    let total_bits = (4 + data.len()) as u64 * 8;
    // 6 bits per pixel
    let pixels_needed = (total_bits + 5) / 6;

    // Add 20% padding for aesthetics
    let target_pixels = (pixels_needed as f64 * 1.2) as u64;

    let side = (target_pixels as f64).sqrt().ceil() as u32;
    let width = side.max(256); // Minimum size 256x256
    let height = side.max(256);

    // 2. Hash data for seed
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let seed = u64::from_le_bytes(result[0..8].try_into().unwrap());

    let mut rng = StdRng::seed_from_u64(seed);

    let mut img = RgbImage::new(width, height);

    // Background: Deep Space
    for pixel in img.pixels_mut() {
        *pixel = Rgb([5, 5, 15]);
    }

    // 3. Draw "Nebula" clouds
    let num_clouds = rng.gen_range(10..30);
    for _ in 0..num_clouds {
        let cx = rng.gen_range(0..width) as f32;
        let cy = rng.gen_range(0..height) as f32;
        let radius = rng.gen_range(20.0..width as f32 / 3.0);

        // Random galaxy colors
        let r_base = rng.gen_range(0..100);
        let g_base = rng.gen_range(0..100);
        let b_base = rng.gen_range(50..200);

        let x_min = (cx - radius).max(0.0) as u32;
        let x_max = (cx + radius).min(width as f32) as u32;
        let y_min = (cy - radius).max(0.0) as u32;
        let y_max = (cy + radius).min(height as f32) as u32;

        for y in y_min..y_max {
            for x in x_min..x_max {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < radius {
                    let alpha = (1.0 - (dist / radius).powf(0.5)).max(0.0);
                    let pixel = img.get_pixel_mut(x, y);
                    let existing = pixel.0;

                    let new_r = (existing[0] as f32 + r_base as f32 * alpha).min(255.0) as u8;
                    let new_g = (existing[1] as f32 + g_base as f32 * alpha).min(255.0) as u8;
                    let new_b = (existing[2] as f32 + b_base as f32 * alpha).min(255.0) as u8;

                    *pixel = Rgb([new_r, new_g, new_b]);
                }
            }
        }
    }

    // 4. Stars
    let num_stars = (width * height) / 200;
    for _ in 0..num_stars {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let brightness = rng.gen_range(150..255);
        img.put_pixel(x, y, Rgb([brightness, brightness, brightness]));
    }

    DynamicImage::ImageRgb8(img)
}
