use anyhow::Result;
use std::f32::consts::PI;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileNode {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub u: f32,
    pub v: f32,
}

pub fn scan_directory(root: &Path, max_depth: usize) -> Result<Vec<FileNode>> {
    let mut nodes = Vec::new();

    // Simple heuristic for mapping
    // We want to group files by directory.
    // Directories are "spines" along the U axis?
    // Or maybe just a cloud.

    // Let's do:
    // u = depth * scale + random_jitter?
    // No, let's make it deterministic.

    for entry in WalkDir::new(root)
        .max_depth(max_depth)
        .into_iter()
        .flatten()
    {
        let depth = entry.depth();
        let file_type = entry.file_type();
        let path_str = entry.path().to_string_lossy().to_string();
        let name = entry.file_name().to_string_lossy().to_string();

        // Map depth to U [0, 2PI].
        // We wrap around if it goes too deep, which is topologically fitting!
        // u goes from 0 to 4PI (double cover) to make it interesting?
        let u = (depth as f32 * 0.5) % (2.0 * PI);

        // Map sibling index to V.
        // Since we are iterating linearly, we don't know the total siblings easily without pre-pass.
        // We can use a hash of the filename or just a running counter.
        // Let's use a hash for stability.

        let hash = path_str
            .bytes()
            .fold(0u64, |acc, b| acc.wrapping_add(b as u64));
        let v_norm = (hash % 1000) as f32 / 1000.0;
        let v = v_norm * 2.0 * PI;

        nodes.push(FileNode {
            path: path_str,
            name,
            is_dir: file_type.is_dir(),
            u,
            v,
        });
    }

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        let nodes = scan_directory(Path::new("."), 1).unwrap();
        assert!(!nodes.is_empty());
        for node in nodes {
            println!("{:?} -> ({}, {})", node.name, node.u, node.v);
            assert!(node.u >= 0.0);
            assert!(node.v >= 0.0 && node.v <= 2.0 * PI);
        }
    }
}
