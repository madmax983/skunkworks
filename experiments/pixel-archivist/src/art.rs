use image::{DynamicImage, Rgb, RgbImage};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    pub path: String,
    pub size: u64,
    pub color: [u8; 3],
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarMap {
    pub stars: Vec<Star>,
}

pub fn calculate_dimensions(data_len: usize) -> (u32, u32) {
    // 4 bytes header + data
    let total_bits = (4 + data_len) as u64 * 8;
    // 6 bits per pixel
    let pixels_needed = (total_bits + 5) / 6;

    // Add 20% padding for aesthetics
    let target_pixels = (pixels_needed as f64 * 1.2) as u64;

    let side = (target_pixels as f64).sqrt().ceil() as u32;
    let width = side.max(512); // Minimum size 512x512
    let height = side.max(512);
    (width, height)
}

pub fn generate_star_map_from_entries(
    entries: &[(String, u64)],
    width: u32,
    height: u32,
) -> StarMap {
    let mut stars = Vec::new();

    // Position determinism uses individual path hash

    for (relative_path, size) in entries {
        // Deterministic position based on path hash
        let mut h = DefaultHasher::new();
        relative_path.hash(&mut h);
        let path_hash = h.finish();
        let mut path_rng = StdRng::seed_from_u64(path_hash);

        // Use padding to avoid edge clipping
        let padding = 20.0;
        let x = path_rng.gen_range(padding..(width as f32 - padding));
        let y = path_rng.gen_range(padding..(height as f32 - padding));

        // Size based on file size (logarithmic)
        let radius = (*size as f64 + 1.0).log2().max(2.0) as f32;

        // Color based on extension
        let path_obj = Path::new(relative_path);
        let ext = path_obj.extension().and_then(|s| s.to_str()).unwrap_or("");
        let color = match ext {
            "rs" => [255, 100, 100],          // Red for Rust
            "toml" => [100, 255, 100],        // Green for Config
            "md" => [100, 100, 255],          // Blue for Docs
            "json" => [255, 255, 100],        // Yellow for Data
            "png" | "jpg" => [255, 100, 255], // Magenta for Images
            "sh" => [100, 255, 255],          // Cyan for Scripts
            _ => [220, 220, 220],             // White for others
        };

        stars.push(Star {
            path: relative_path.clone(),
            size: *size,
            color,
            x,
            y,
            radius,
        });
    }
    StarMap { stars }
}

pub fn generate_star_map(dir: &Path, width: u32, height: u32) -> StarMap {
    let walker = WalkDir::new(dir).into_iter();
    let mut entries = Vec::new();

    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            // Store relative path
            let relative_path = match path.strip_prefix(dir) {
                Ok(p) => p.to_string_lossy().to_string(),
                Err(_) => path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            };
            let metadata = entry.metadata().ok();
            let size = metadata.map(|m| m.len()).unwrap_or(0);
            entries.push((relative_path, size));
        }
    }
    generate_star_map_from_entries(&entries, width, height)
}

pub fn generate_cover(
    data: &[u8],
    width: u32,
    height: u32,
    starmap: Option<&StarMap>,
) -> DynamicImage {
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
    if let Some(map) = starmap {
        for star in &map.stars {
            let cx = star.x.round() as i32;
            let cy = star.y.round() as i32;
            let r = star.radius.ceil() as i32;
            let color = Rgb(star.color);

            // Draw circle
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx * dx + dy * dy <= r * r {
                        let px = cx + dx;
                        let py = cy + dy;
                        if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                            img.put_pixel(px as u32, py as u32, color);
                        }
                    }
                }
            }
        }
    } else {
        let num_stars = (width * height) / 200;
        for _ in 0..num_stars {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            let brightness = rng.gen_range(150..255);
            img.put_pixel(x, y, Rgb([brightness, brightness, brightness]));
        }
    }

    DynamicImage::ImageRgb8(img)
}
