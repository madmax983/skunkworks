use macroquad::prelude::Color;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub size: u64,
}

pub struct Scanner;

impl Scanner {
    pub fn scan<P: AsRef<Path>>(root: P) -> Vec<FileMetadata> {
        let root = root.as_ref();
        let mut files = Vec::new();
        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                // Ignore .git folder
                if entry.path().components().any(|c| c.as_os_str() == ".git") {
                    continue;
                }

                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                // Strip prefix to match git paths (which are relative to repo root)
                let path = entry
                    .path()
                    .strip_prefix(root)
                    .unwrap_or(entry.path())
                    .to_path_buf();

                files.push(FileMetadata { path, size });
            }
        }
        // Sort alphabetically to group directories
        files.sort_by(|a, b| a.path.cmp(&b.path));
        files
    }
}

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    pub heightmap: Vec<f32>,
    pub colors: Vec<Color>,
    pub max_height: f32,
    pub file_indices: HashMap<PathBuf, usize>,
}

impl Terrain {
    pub fn from_files(files: &[FileMetadata]) -> Self {
        let n = files.len();
        // Find grid size: nearest power of 2 that fits sqrt(N)
        let side = (n as f64).sqrt().ceil() as usize;
        let mut power_of_two = 1;
        while power_of_two < side {
            power_of_two *= 2;
        }
        let width = power_of_two;
        let height = power_of_two;

        let mut heightmap = vec![0.0; width * height];
        let colors = vec![Color::new(1.0, 1.0, 1.0, 1.0); width * height];
        let mut file_indices = HashMap::new();
        let mut max_height = 0.0;

        for (i, file) in files.iter().enumerate() {
            if i >= width * height {
                break;
            }
            let (x, y) = d2xy(width, i);
            // Log scale for size to avoid massive spikes
            let h = (file.size as f32).ln().max(0.0) * 2.0;
            heightmap[y * width + x] = h;
            file_indices.insert(file.path.clone(), i);
            if h > max_height {
                max_height = h;
            }
        }

        Self {
            width,
            height,
            heightmap,
            colors,
            max_height,
            file_indices,
        }
    }

    pub fn get_coords(&self, path: &Path) -> Option<(usize, usize)> {
        self.file_indices.get(path).map(|&i| d2xy(self.width, i))
    }
}

// Hilbert Curve Mapping
// N must be a power of 2
pub fn d2xy(n: usize, mut d: usize) -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;
    let mut s = 1;
    while s < n {
        let rx = 1 & (d / 2);
        let ry = 1 & (d ^ rx);
        rot(s, &mut x, &mut y, rx, ry);
        x += s * rx;
        y += s * ry;
        d /= 4;
        s *= 2;
    }
    (x, y)
}

fn rot(n: usize, x: &mut usize, y: &mut usize, rx: usize, ry: usize) {
    if ry == 0 {
        if rx == 1 {
            *x = n - 1 - *x;
            *y = n - 1 - *y;
        }
        // Swap x and y
        let t = *x;
        *x = *y;
        *y = t;
    }
}
