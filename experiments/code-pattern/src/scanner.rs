use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct FileMetric {
    pub path: String,
    pub size: u64,
    pub depth: usize,
    pub complexity: f32, // Normalized 0.0-1.0
    pub hash_val: f32,   // Normalized 0.0-1.0 from hash
}

pub struct Scanner;

impl Scanner {
    pub fn scan_directory(root: &str) -> Vec<FileMetric> {
        let mut metrics = Vec::new();
        let walker = WalkDir::new(root).max_depth(10);

        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(metric) = Self::scan_file(entry.path(), root) {
                    metrics.push(metric);
                }
            }
        }
        metrics
    }

    fn scan_file(path: &Path, root: &str) -> std::io::Result<FileMetric> {
        let metadata = fs::metadata(path)?;
        let size = metadata.len();

        // Calculate depth relative to root
        let root_path = Path::new(root);
        let depth = path.strip_prefix(root_path).unwrap_or(path).components().count();

        // Hash path for stability
        let mut hasher = Sha256::new();
        hasher.update(path.to_string_lossy().as_bytes());
        let result = hasher.finalize();
        let hash_byte = result[0];
        let hash_val = hash_byte as f32 / 255.0;

        // Complexity heuristic (fake for now, could count lines)
        let complexity = (size as f32 / 10000.0).clamp(0.0, 1.0);

        Ok(FileMetric {
            path: path.to_string_lossy().to_string(),
            size,
            depth,
            complexity,
            hash_val,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        // Just check it doesn't panic
        let _metrics = Scanner::scan_directory("src");
        // We might not have files in src if running from root, but usually yes.
    }
}
