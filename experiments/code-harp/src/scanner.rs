use macroquad::prelude::*;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct StringEntity {
    pub path: String,
    pub frequency: f32, // Hz
    pub damping: f32,   // 0.0 - 1.0 (1.0 = no decay)
    pub start: Vec3,
    pub end: Vec3,
    pub color: Color,
    pub size: u64,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with('.') || s == "target" || s == "node_modules")
         .unwrap_or(false)
}

pub fn scan_codebase(root: &str) -> Vec<StringEntity> {
    let mut strings = Vec::new();

    // WalkDir::new returns a builder. into_iter() gives us the iterator which has filter_entry.
    // We want to skip hidden directories entirely.
    let walker = WalkDir::new(root).into_iter().filter_entry(|e| !is_hidden(e));

    // Mapping parameters: Spiral Cylinder
    let mut angle = 0.0f32;
    let radius = 10.0f32;
    let height_step = 0.05f32; // Vertical distance between strings
    let mut current_height = -20.0f32;

    for entry in walker {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                let path = entry.path();
                let path_str = path.to_string_lossy().to_string();

                let metadata = entry.metadata().ok();
                let size = metadata.map(|m: std::fs::Metadata| m.len()).unwrap_or(1000);

                // Frequency mapping: Smaller = Higher pitch
                // Base frequency range: 100Hz to 800Hz
                // f = C / (size^k)
                // Let's use a simple inverse mapping.
                // clamp size to avoid div by zero or huge frequencies.
                let clamped_size = (size as f32).clamp(100.0, 100000.0);
                let frequency = 200.0 + (50000.0 / clamped_size);
                // Small file (100 bytes) -> 200 + 500 = 700Hz
                // Large file (100k bytes) -> 200 + 0.5 = 200.5Hz

                // Damping:
                // Rust strings are high quality -> high resonance (close to 1.0)
                // Configs are dull -> lower resonance
                let damping = match path.extension().and_then(|s: &std::ffi::OsStr| s.to_str()) {
                    Some("rs") => 0.995,
                    Some("toml") => 0.980,
                    Some("md") => 0.990,
                    Some("json") => 0.985,
                    _ => 0.992,
                };

                // Position mapping
                // Spiral layout
                let x = radius * angle.cos();
                let z = radius * angle.sin();
                let y_start = current_height;
                // String length is purely visual here, but could relate to size.
                let length = 2.0 + (size as f32 / 1000.0).clamp(0.0, 5.0);
                let y_end = current_height + length;

                let color = match path.extension().and_then(|s: &std::ffi::OsStr| s.to_str()) {
                    Some("rs") => ORANGE,
                    Some("toml") => GREEN,
                    Some("md") => SKYBLUE,
                    Some("json") => YELLOW,
                    Some("lock") => DARKGRAY,
                    _ => LIGHTGRAY,
                };

                strings.push(StringEntity {
                    path: path_str,
                    frequency,
                    damping,
                    start: vec3(x, y_start, z),
                    end: vec3(x, y_end, z),
                    color,
                    size,
                });

                angle += 0.3; // Spiral
                current_height += height_step;
            }
        }
    }

    // Center the spiral vertically
    let center_offset = current_height / 2.0;
    for s in &mut strings {
        s.start.y -= center_offset;
        s.end.y -= center_offset;
    }

    println!("Scanned {} strings.", strings.len());

    strings
}
