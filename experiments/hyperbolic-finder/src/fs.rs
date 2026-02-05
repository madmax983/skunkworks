use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DirNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<DirNode>,
    pub size: u64, // In bytes, 0 for directories (unless we aggregate)
}

impl DirNode {
    pub fn new(path: PathBuf, is_dir: bool, size: u64) -> Self {
        let name = path
            .file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string();
        Self {
            path,
            name,
            is_dir,
            children: Vec::new(),
            size,
        }
    }
}

pub fn scan_dir<P: AsRef<Path>>(path: P, max_depth: usize) -> Result<DirNode> {
    let path = path.as_ref();
    let metadata = fs::metadata(path)?;
    let mut node = DirNode::new(path.to_path_buf(), metadata.is_dir(), metadata.len());

    if max_depth > 0 && node.is_dir {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let child_path = entry.path();
                        // Ignore hidden files for sanity
                        if child_path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .map(|s| s.starts_with('.'))
                            .unwrap_or(false)
                        {
                            continue;
                        }

                        if let Ok(child) = scan_dir(&child_path, max_depth - 1) {
                            node.children.push(child);
                        }
                    }
                }
            }
            Err(_) => {
                // Ignore permission errors etc
            }
        }
    }
    // Sort children: directories first, then alphabetical
    node.children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    Ok(node)
}
