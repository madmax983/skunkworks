use anyhow::Result;
use bevy::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Component)]
pub struct CodeString {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub extension: String,
}

pub fn scan_directory(root: &str) -> Result<Vec<FileInfo>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let metadata = entry.metadata()?;
            let size = metadata.len();
            // Skip huge files or tiny files if needed
            if size > 0 {
                let path = entry.path().to_path_buf();
                let extension = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_string();

                files.push(FileInfo {
                    path,
                    size,
                    extension,
                });
            }
        }
    }

    // Sort by path for consistency
    files.sort_by(|a, b| a.path.cmp(&b.path));

    // Limit to top 200 to avoid performance death for now
    if files.len() > 200 {
        files.truncate(200);
    }

    Ok(files)
}

pub fn get_frequency_from_size(size: u64) -> f32 {
    // Inverse relationship: larger file = lower pitch
    // Base frequency 100Hz for 1KB file?
    // f = C / size
    // Let's use a pentatonic scale mapping to sound musical.
    // Map size to midi note, then to frequency.

    let base_note = 60.0; // Middle C
    let size_factor = (size as f64).log2(); // Logarithmic size

    // Map log size to note offset
    // Small file (100 bytes) -> log2(100) = 6.6 -> High pitch
    // Large file (1MB) -> log2(1000000) = 20 -> Low pitch

    let note = base_note - (size_factor - 10.0) * 2.0;
    let note = note.clamp(30.0, 90.0);

    // Quantize to pentatonic (Major: 0, 2, 4, 7, 9)
    let octave = (note / 12.0).floor();
    let semi = note % 12.0;

    let quant_semi = match semi as i32 {
        0..=1 => 0.0,
        2..=3 => 2.0,
        4..=6 => 4.0,
        7..=8 => 7.0,
        _ => 9.0,
    };

    let quantized_note = octave * 12.0 + quant_semi;

    440.0 * 2.0f32.powf((quantized_note as f32 - 69.0) / 12.0)
}

pub fn get_color_from_ext(ext: &str) -> Color {
    match ext {
        "rs" => Color::srgb(1.0, 0.3, 0.0), // Rust Orange
        "toml" => Color::srgb(0.5, 0.5, 0.5), // Gray
        "md" => Color::srgb(0.0, 0.5, 1.0), // Blue
        "json" => Color::srgb(1.0, 1.0, 0.0), // Yellow
        "lock" => Color::srgb(1.0, 0.0, 0.0), // Red
        "png" | "jpg" => Color::srgb(0.8, 0.0, 0.8), // Purple
        _ => Color::srgb(0.0, 1.0, 0.0), // Green default
    }
}
