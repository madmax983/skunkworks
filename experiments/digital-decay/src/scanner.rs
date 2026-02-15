use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct FileNode {
    pub path: PathBuf,
    pub size: u64,
    pub modified: SystemTime,
    pub health: f32, // 0.0 to 1.0
}

pub fn scan_dir(root: &Path) -> Vec<FileNode> {
    let mut files = Vec::new();

    // Scan recursively
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                files.push(FileNode {
                    path: entry.path().to_path_buf(),
                    size: metadata.len(),
                    modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                    health: 1.0,
                });
            }
        }
    }

    files
}
