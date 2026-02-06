use walkdir::WalkDir;
use std::collections::HashMap;
use std::fs;

pub const GRID_SIZE: usize = 256;

pub struct Terrain {
    pub heightmap: Vec<f32>,
    pub sediment_map: Vec<f32>,
    pub file_map: HashMap<String, (usize, usize)>,
}

impl Terrain {
    pub fn new() -> Self {
        let size = GRID_SIZE * GRID_SIZE;
        Self {
            heightmap: vec![0.0; size],
            sediment_map: vec![0.0; size],
            file_map: HashMap::new(),
        }
    }

    pub fn init_from_files(&mut self, root: &str) {
        println!("Scanning files in {}...", root);
        let mut files = Vec::new();

        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                // Ignore target, .git
                let path_str = entry.path().to_string_lossy().to_string();
                if path_str.contains("/target/") || path_str.contains("/.git/") {
                    continue;
                }
                files.push(path_str);
            }
        }

        // Sort for stability
        files.sort();
        println!("Found {} files.", files.len());

        // Map to grid
        // Scanline mapping
        for (i, path) in files.iter().enumerate() {
            if i >= GRID_SIZE * GRID_SIZE {
                break;
            }

            let x = i % GRID_SIZE;
            let y = i / GRID_SIZE;

            self.file_map.insert(path.clone(), (x, y));

            // Set initial height based on file size
            if let Ok(metadata) = fs::metadata(path) {
                let size = metadata.len();
                // Height scaling: log scale or sqrt?
                // 1KB -> 1.0 height
                // 1MB -> 1000.0 height? Too tall.
                // Let's take sqrt(size) / 10.0
                let h = (size as f32).sqrt() / 5.0;
                self.heightmap[y * GRID_SIZE + x] = h.max(0.5); // At least some height
            }
        }
    }
    pub fn get_height(&self, x: usize, y: usize) -> f32 {
        if x >= GRID_SIZE || y >= GRID_SIZE { return 0.0; }
        self.heightmap[y * GRID_SIZE + x]
    }

    pub fn get_coords(&self, path: &str) -> Option<(usize, usize)> {
        // Try exact match first
        if let Some(coords) = self.file_map.get(path) {
            return Some(*coords);
        }

        // Try relative path match if input path is relative and map has absolute, or vice versa
        // This is tricky. Let's just rely on exact match for now, or suffix match.
        // git log output paths are relative to repo root (e.g. "experiments/code-erosion/src/main.rs")
        // WalkDir output might be "./experiments..."

        let needle = if path.starts_with("./") {
            &path[2..]
        } else {
            path
        };

        for (key, val) in &self.file_map {
            // key might be "./experiments/..."
            let key_clean = if key.starts_with("./") {
                &key[2..]
            } else {
                key
            };

            if key_clean == needle {
                return Some(*val);
            }
        }

        None
    }
}
