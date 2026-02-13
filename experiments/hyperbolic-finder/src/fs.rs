use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Directory,
    Code,
    Image,
    Audio,
    Video,
    Archive,
    Text,
    Other,
}

impl FileType {
    pub fn from_path(path: &Path, is_dir: bool) -> Self {
        if is_dir {
            return FileType::Directory;
        }
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            match ext.to_lowercase().as_str() {
                "rs" | "py" | "js" | "ts" | "c" | "cpp" | "h" | "go" | "java" | "html" | "css"
                | "toml" | "json" | "yaml" | "sh" | "lua" => FileType::Code,
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" => FileType::Image,
                "wav" | "mp3" | "ogg" | "flac" | "aac" => FileType::Audio,
                "mp4" | "mkv" | "avi" | "mov" | "webm" => FileType::Video,
                "zip" | "tar" | "gz" | "7z" | "rar" => FileType::Archive,
                "txt" | "md" | "csv" | "log" => FileType::Text,
                _ => FileType::Other,
            }
        } else {
            FileType::Other
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub file_type: FileType,
    pub children: Vec<DirNode>,
    /// Size of the file itself (bytes). For directories, typically 4096.
    pub self_size: u64,
    /// Total size of this node + all recursive children (bytes).
    pub total_size: u64,
}

impl DirNode {
    pub fn new(path: PathBuf, is_dir: bool, self_size: u64) -> Self {
        let name = path
            .file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string();
        let file_type = FileType::from_path(&path, is_dir);
        Self {
            path,
            name,
            is_dir,
            file_type,
            children: Vec::new(),
            self_size,
            total_size: self_size,
        }
    }
}

pub fn scan_dir<P: AsRef<Path>>(path: P, max_depth: usize) -> Result<DirNode> {
    let path = path.as_ref();
    let metadata = fs::metadata(path)?;
    let mut node = DirNode::new(path.to_path_buf(), metadata.is_dir(), metadata.len());

    if max_depth > 0 && node.is_dir {
        // Read directory entries
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let child_path = entry.path();
                        // Ignore hidden files
                        if child_path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .map(|s| s.starts_with('.'))
                            .unwrap_or(false)
                        {
                            continue;
                        }

                        // Recursively scan children
                        // We use a Result here but swallow errors for individual children
                        // so one bad permission doesn't stop the whole scan.
                        if let Ok(child) = scan_dir(&child_path, max_depth - 1) {
                            node.children.push(child);
                        }
                    }
                }
            }
            Err(_) => {
                // Permission denied or other error reading dir: treat as empty dir
            }
        }
    }

    // Calculate total size
    // Note: We sum children's total_size, not self_size, because children might be dirs with content.
    for child in &node.children {
        node.total_size += child.total_size;
    }

    // Sort children: directories first, then by size descending
    node.children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        // Within same type, sort by size descending
        _ => b.total_size.cmp(&a.total_size),
    });

    Ok(node)
}
