use crate::model::{Platter, Sector};
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn scan_and_populate(path: &Path, platter: &mut Platter) -> Result<()> {
    let mut files = Vec::new();

    // Read directory
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            files.push(path);
        }
    }

    // Sort to be deterministic
    files.sort();

    // Map files to UV space
    // Similar to klein-fs layout
    let count = files.len();
    if count == 0 {
        return Ok(());
    }

    for (i, file_path) in files.iter().enumerate() {
        let t = i as f32 / count as f32;
        // u goes from 0 to 2PI
        let u = t * std::f32::consts::PI * 2.0;
        // v spirals 3 times
        let v = t * std::f32::consts::PI * 2.0 * 3.0;

        let name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // Read data
        let mut data = [0u8; 64];
        if file_path.is_file() {
            if let Ok(mut f) = File::open(file_path) {
                let _ = f.read_exact(&mut data); // Ignore error (partial read is fine, it leaves 0s)
            }
        } else {
            // Fill with some pattern for directories
            for j in 0..64 {
                data[j] = (j as u8).wrapping_mul(i as u8);
            }
        }

        let sector = Sector::new(u, v, data, name);
        platter.add_sector(sector);
    }

    Ok(())
}
