use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy)]
pub struct CrystalPoint {
    pub position: [f32; 4],
    pub color: [f32; 4],
}

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn get_vector_from_seed(seed: u64) -> [f32; 4] {
    let mut rng = StdRng::seed_from_u64(seed);
    // Generate a random unit vector in 4D
    let mut v: [f32; 4] = [
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    ];

    // Normalize
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2] + v[3] * v[3]).sqrt();
    if len > 0.0001 {
        v[0] /= len;
        v[1] /= len;
        v[2] /= len;
        v[3] /= len;
    }
    v
}

fn get_color_from_ext(ext: Option<&str>) -> [f32; 4] {
    match ext {
        Some("rs") => [1.0, 0.2, 0.0, 1.0],   // Rust - Orange
        Some("toml") => [0.2, 0.8, 0.2, 1.0], // Config - Green
        Some("md") => [0.2, 0.2, 0.8, 1.0],   // Markdown - Blue
        Some("lock") => [0.5, 0.5, 0.5, 1.0], // Lock - Grey
        Some("png") | Some("jpg") => [0.8, 0.2, 0.8, 1.0], // Images - Purple
        _ => [0.9, 0.9, 0.9, 1.0],            // Default - White
    }
}

pub fn scan(root: &Path) -> Vec<CrystalPoint> {
    let mut points = Vec::new();

    // We need to track the accumulated vector for each directory.
    // Since WalkDir is recursive-ish (it flattens), we can recompute path vectors.
    // Optimally we'd maintain a stack, but path reconstruction is cheap enough for now.

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Calculate position by summing vectors of path components relative to root
        let relative_path = path.strip_prefix(root).unwrap_or(path);

        let mut pos: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        for component in relative_path.components() {
            let s = component.as_os_str().to_string_lossy();
            let seed = hash_string(&s);
            let vec = get_vector_from_seed(seed);

            pos[0] += vec[0];
            pos[1] += vec[1];
            pos[2] += vec[2];
            pos[3] += vec[3];
        }

        // If it's a file, add it as a point
        if entry.file_type().is_file() {
            let ext = path.extension().and_then(|s| s.to_str());
            let color = get_color_from_ext(ext);
            points.push(CrystalPoint {
                position: pos,
                color,
            });
        }
        // If it's a directory, maybe add a node point too?
        // Let's add directory points as larger/different colored?
        // For now, just files.
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_scanner_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.rs");
        File::create(&file_path).unwrap();

        let points = scan(dir.path());
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].color, [1.0, 0.2, 0.0, 1.0]); // Rust color
    }

    #[test]
    fn test_determinism() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("a/b/c");
        std::fs::create_dir_all(&p1).unwrap();
        File::create(p1.join("file.txt")).unwrap();

        let points1 = scan(dir.path());
        let points2 = scan(dir.path());

        assert_eq!(points1.len(), 1);
        assert_eq!(points1[0].position, points2[0].position);
    }
}
