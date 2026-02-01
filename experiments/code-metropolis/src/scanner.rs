use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Node {
    File {
        path: PathBuf,
        size: u64,
    },
    Dir {
        path: PathBuf,
        children: Vec<Node>,
    },
}

impl Node {
    pub fn size(&self) -> u64 {
        match self {
            Node::File { size, .. } => *size,
            Node::Dir { children, .. } => children.iter().map(|c| c.size()).sum(),
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, Node::Dir { .. })
    }
}

pub fn scan(path: impl AsRef<Path>) -> Result<Node> {
    let path = path.as_ref();
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        return Ok(Node::File {
            path: path.to_path_buf(),
            size: metadata.len(),
        });
    }

    let mut children = Vec::new();
    if metadata.is_dir() {
        // Read dir, ignore errors for individual files (e.g. permissions)
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                // Skip hidden files (starting with .)
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with('.') {
                        continue;
                    }
                }

                if let Ok(node) = scan(entry.path()) {
                    children.push(node);
                }
            }
        }
    }

    Ok(Node::Dir {
        path: path.to_path_buf(),
        children,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_scan_basic() {
        let test_dir = "target/test_scan";
        // Clean start
        let _ = fs::remove_dir_all(test_dir);
        fs::create_dir_all(test_dir).unwrap();

        fs::write(format!("{}/file1.txt", test_dir), "hello").unwrap();
        fs::create_dir(format!("{}/subdir", test_dir)).unwrap();
        fs::write(format!("{}/subdir/file2.txt", test_dir), "world").unwrap();

        let node = scan(test_dir).unwrap();

        if let Node::Dir { children, .. } = node {
             // Should have file1 and subdir
             assert_eq!(children.len(), 2);

             let total_size = children.iter().map(|c| c.size()).sum::<u64>();
             assert_eq!(total_size, 10); // 5 + 5 bytes
        } else {
             panic!("Root should be dir");
        }

        // Clean up
        let _ = fs::remove_dir_all(test_dir);
    }
}
