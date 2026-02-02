use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

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
    let mut ancestors = HashSet::new();
    scan_recursive(path, depth, &mut ancestors)
}

fn scan_recursive(path: &Path, depth: usize, ancestors: &mut HashSet<PathBuf>) -> Result<FsNode> {
    // Canonicalize to detect loops
    let canonical = path.canonicalize().unwrap_or(path.to_path_buf());

    // Check cycle
    if ancestors.contains(&canonical) {
        return Ok(FsNode {
            path: path.to_path_buf(),
            is_dir: path.is_dir(),
            children: Vec::new(),
        });
    }

    ancestors.insert(canonical.clone());

    let is_dir = path.is_dir();
    let mut children = Vec::new();

    if is_dir && depth > 0 {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if let Ok(node) = scan_recursive(&entry_path, depth - 1, ancestors) {
                    children.push(node);
                }
            }
        }
        // Sort children for consistent layout
        children.sort_by(|a, b| a.path.cmp(&b.path));
    }

    ancestors.remove(&canonical);

    Ok(FsNode {
        path: path.to_path_buf(),
        is_dir,
        children,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};

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

        let sub_node = root
            .children
            .iter()
            .find(|n| n.is_dir)
            .expect("Sub dir not found");
        assert_eq!(sub_node.children.len(), 1); // file2.txt

        // Cleanup
        let _ = fs::remove_dir_all(&temp_path);
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn test_symlink_cycle_detection() -> Result<()> {
        let mut temp_path = env::temp_dir();
        temp_path.push("hyperbolic_test_cycle");
        let _ = fs::remove_dir_all(&temp_path);
        fs::create_dir_all(&temp_path)?;

        let dir_a = temp_path.join("a");
        fs::create_dir(&dir_a)?;

        // Create symlink: temp_path/a/link -> temp_path
        std::os::unix::fs::symlink(&temp_path, dir_a.join("link"))?;

        // Scan depth 3
        // root -> a -> link (points to root)
        // If cycle detection works, 'link' should be visited but its children should NOT be scanned
        // because scanning them would mean re-scanning 'root', which is already in the stack/visited.

        let root = scan_depth(&temp_path, 3)?;

        let node_a = root
            .children
            .iter()
            .find(|n| n.name() == "a")
            .expect("a not found");
        let node_link = node_a
            .children
            .iter()
            .find(|n| n.name() == "link")
            .expect("link not found");

        // Assert that the link node has no children (recursion stopped)
        // Current implementation will fail this because it will find 'a' inside 'link'
        assert!(
            node_link.children.is_empty(),
            "Cycle not detected: link node has children"
        );

        let _ = fs::remove_dir_all(&temp_path);
        Ok(())
    }
}
