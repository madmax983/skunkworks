use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct FsNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<FsNode>,
}

impl FsNode {
    pub fn name(&self) -> String {
        self.path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| self.path.to_string_lossy().to_string())
    }
}

pub fn scan_depth(path: &Path, depth: usize) -> Result<FsNode> {
    let is_dir = path.is_dir();
    let mut children = Vec::new();

    if is_dir && depth > 0 {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                // Basic symlink loop protection: don't follow if we've seen it?
                // For MVP, just don't worry about cycles yet.

                // Recursively scan
                if let Ok(node) = scan_depth(&entry_path, depth - 1) {
                    children.push(node);
                }
            }
        }
        // Sort children for consistent layout
        children.sort_by(|a, b| a.path.cmp(&b.path));
    }

    Ok(FsNode {
        path: path.to_path_buf(),
        is_dir,
        children,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::env;

    #[test]
    fn test_scan_depth() -> Result<()> {
        // Create a temp dir structure
        // /tmp/hyperbolic_test/
        //   - file1.txt
        //   - sub/
        //     - file2.txt

        let mut temp_path = env::temp_dir();
        temp_path.push("hyperbolic_test_fs");
        let _ = fs::remove_dir_all(&temp_path); // Cleanup previous runs
        fs::create_dir_all(&temp_path)?;

        File::create(temp_path.join("file1.txt"))?;
        let sub = temp_path.join("sub");
        fs::create_dir(&sub)?;
        File::create(sub.join("file2.txt"))?;

        // Scan depth 2
        let root = scan_depth(&temp_path, 2)?;
        assert!(root.is_dir);
        assert_eq!(root.children.len(), 2); // file1.txt, sub

        let sub_node = root.children.iter().find(|n| n.is_dir).expect("Sub dir not found");
        assert_eq!(sub_node.children.len(), 1); // file2.txt

        // Cleanup
        let _ = fs::remove_dir_all(&temp_path);
        Ok(())
    }
}
