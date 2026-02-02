use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct TerrainMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f64>,
}

impl TerrainMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.data[y * self.width + x]
    }

    pub fn get_f64(&self, x: f64, y: f64) -> f64 {
        let x_i = x.floor() as usize;
        let y_i = y.floor() as usize;
        // Bilinear interpolation could go here, but for now let's just do nearest neighbor
        // or simple bounds check to prevent panic
        self.get(x_i, y_i)
    }

    pub fn set(&mut self, x: usize, y: usize, val: f64) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = val;
        }
    }

    pub fn set_f64(&mut self, x: f64, y: f64, val: f64) {
        let x_i = x.floor() as usize;
        let y_i = y.floor() as usize;
        self.set(x_i, y_i, val);
    }

    pub fn get_gradient(&self, x: f64, y: f64) -> (f64, f64) {
        let x_i = x.floor() as usize;
        let y_i = y.floor() as usize;

        let h00 = self.get(x_i, y_i);
        let h10 = self.get(x_i + 1, y_i);
        let h01 = self.get(x_i, y_i + 1);

        // Simple forward difference
        let gx = h10 - h00;
        let gy = h01 - h00;
        (gx, gy)
    }

    /// Generates a terrain map from the file system.
    /// Files are mapped to grid cells. Height is determined by file size.
    pub fn from_filesystem(path: impl AsRef<Path>) -> Result<Self> {
        let mut files = Vec::new();

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            // Skip hidden files/dirs and target directory
            if path.to_string_lossy().contains("/.") || path.to_string_lossy().contains("\\.") {
                continue;
            }
            if path.components().any(|c| c.as_os_str() == "target") {
                continue;
            }

            if entry.file_type().is_file() {
                if let Ok(metadata) = entry.metadata() {
                    files.push(metadata.len());
                }
            }
        }

        let count = files.len();
        if count == 0 {
            return Ok(Self::new(10, 10)); // Return empty terrain if no files
        }

        // Calculate grid dimensions (roughly square)
        let side = (count as f64).sqrt().ceil() as usize;
        let width = side;
        let height = side;

        let mut map = Self::new(width, height);

        // Normalize heights slightly?
        // Let's just use raw size for now, maybe log scale if it's too crazy.
        // Actually log scale is probably better for visual variation.
        for (i, &size) in files.iter().enumerate() {
            let x = i % width;
            let y = i / width;

            // Log scale: log2(size + 1) to handle 0 size
            let h = (size as f64 + 1.0).log2();
            map.set(x, y, h);
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_filesystem() {
        // Use the current crate's src directory
        let path = Path::new("src");
        let map = TerrainMap::from_filesystem(path).expect("Failed to create terrain");

        assert!(map.width > 0);
        assert!(map.height > 0);
        assert!(map.data.len() == map.width * map.height);

        // We know there are at least 2 files (main.rs, terrain.rs)
        // so heightmap should have some non-zero values.
        let non_zeros = map.data.iter().filter(|&&h| h > 0.0).count();
        assert!(non_zeros >= 2, "Expected at least 2 non-zero heights, found {}", non_zeros);
    }
}
