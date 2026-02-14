use std::f32::consts::PI;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub u: f32,
    pub v: f32,
    pub depth: usize,
}

pub fn scan_fs(root: &Path) -> Vec<FileNode> {
    let mut nodes = Vec::new();
    let walker = WalkDir::new(root).into_iter();

    let mut raw_entries = Vec::new();
    for entry in walker.filter_map(|e| e.ok()) {
        raw_entries.push(entry);
    }

    let total = raw_entries.len() as f32;
    // Heuristic: We want the spiral to wrap around the tube as we traverse the figure-8.
    // The number of wraps should be proportional to the square root of files to distribute them evenly?
    // Or just a fixed density.
    let loops = (total.sqrt() / 2.0).max(5.0);

    for (i, entry) in raw_entries.iter().enumerate() {
        let t = i as f32 / total; // 0.0 to 1.0

        // u traverses the Figure-8 (0 to 2PI)
        let u = t * PI * 2.0;

        // v traverses the tube cross-section (wraps around 'loops' times)
        let v = t * PI * 2.0 * loops;

        nodes.push(FileNode {
            path: entry.path().to_path_buf(),
            is_dir: entry.file_type().is_dir(),
            u,
            v,
            depth: entry.depth(),
        });
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_fs() {
        let nodes = scan_fs(Path::new("."));
        assert!(!nodes.is_empty(), "Should find files in current directory");

        let first = &nodes[0];
        assert_eq!(first.u, 0.0);
        assert_eq!(first.v, 0.0);

        if nodes.len() > 1 {
            let last = &nodes[nodes.len() - 1];
            // u should be close to 2PI (but not quite, since i goes to total-1)
            // t = (total-1)/total approx 1.0
            assert!(last.u > 0.0);
        }
    }
}
