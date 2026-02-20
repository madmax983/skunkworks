use anyhow::Result;
use git2::Repository;
use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitStatus {
    New,
    Modified,
    Deleted,
    Renamed,
    Ignored,
    Clean,
    Conflict,
}

#[derive(Debug, Clone)]
pub struct DirNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub file_type: FileType,
    pub children: Vec<DirNode>,
    /// Size of the file itself (bytes). For directories, typically 4096.
    #[allow(dead_code)]
    pub self_size: u64,
    /// Total size of this node + all recursive children (bytes).
    pub total_size: u64,
    pub git_status: Option<GitStatus>,
}

impl DirNode {
    pub fn new(path: PathBuf, is_dir: bool, self_size: u64, git_status: Option<GitStatus>) -> Self {
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
            git_status,
        }
    }
}

pub fn get_repo_statuses(root: &Path) -> HashMap<PathBuf, GitStatus> {
    let mut map = HashMap::new();
    // Try to find repo starting from root
    if let Ok(repo) = Repository::discover(root) {
        if let Ok(statuses) = repo.statuses(None) {
            for entry in statuses.iter() {
                if let Some(path_str) = entry.path() {
                    // entry.path() is relative to repo workdir
                    if let Some(workdir) = repo.workdir() {
                        let full_path = workdir.join(path_str);

                        let status = entry.status();
                        let s = if status.is_conflicted() {
                            GitStatus::Conflict
                        } else if status.is_wt_new() || status.is_index_new() {
                            GitStatus::New
                        } else if status.is_wt_modified() || status.is_index_modified() {
                            GitStatus::Modified
                        } else if status.is_wt_deleted() || status.is_index_deleted() {
                            GitStatus::Deleted
                        } else if status.is_ignored() {
                            GitStatus::Ignored
                        } else if status.is_index_renamed() {
                            GitStatus::Renamed
                        } else {
                            // clean
                            GitStatus::Clean
                        };

                        // We only care if it's not clean, but map.insert overwrites
                        if s != GitStatus::Clean {
                            map.insert(full_path, s);
                        }
                    }
                }
            }
        }
    }
    map
}

pub fn scan_dir<P: AsRef<Path>>(
    path: P,
    max_depth: usize,
    git_map: &HashMap<PathBuf, GitStatus>,
) -> Result<DirNode> {
    let path = path.as_ref();
    let metadata = fs::metadata(path)?;

    // Check if path is in git_map
    // We need to be careful with canonicalization if map has canonical paths,
    // but here we constructed map with workdir.join(rel), which is usually absolute.
    // Let's assume path passed in is also compatible (e.g. absolute).
    // If not, we might miss matches.
    // Best practice: use canonical paths for keys, but canonicalize can be slow/fail.
    // For now, rely on consistent path usage (absolute).
    let path_buf = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };

    let status = git_map.get(&path_buf).copied();

    let mut node = DirNode::new(
        path.to_path_buf(),
        metadata.is_dir(),
        metadata.len(),
        status,
    );

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
                        if let Ok(child) = scan_dir(&child_path, max_depth - 1, git_map) {
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

pub fn get_view_root(
    path: &Path,
    max_depth: usize,
    git_map: &HashMap<PathBuf, GitStatus>,
) -> Result<DirNode> {
    let mut root = scan_dir(path, max_depth, git_map)?;

    // Add ".." if parent exists
    if let Some(parent) = path.parent() {
        let parent_node = DirNode {
            path: parent.to_path_buf(),
            name: "..".to_string(),
            is_dir: true,
            file_type: FileType::Directory,
            children: Vec::new(),
            self_size: 0,
            total_size: 0, // Keep it small so it doesn't dominate layout
            git_status: None,
        };
        // Insert at beginning
        root.children.insert(0, parent_node);
    }

    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_git_status_integration() -> Result<()> {
        // Create a temp git repo
        let dir = tempdir()?;
        let root = dir.path();
        let repo = Repository::init(root)?;

        // Create a file (New)
        let file_path = root.join("new_file.txt");
        let mut file = File::create(&file_path)?;
        writeln!(file, "Hello")?;

        // Create a file and commit it (Clean)
        let clean_path = root.join("clean_file.txt");
        let mut file = File::create(&clean_path)?;
        writeln!(file, "Clean")?;

        // Commit
        let mut index = repo.index()?;
        index.add_path(Path::new("clean_file.txt"))?;
        index.write()?; // Write index to disk!
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let sig = repo.signature()?;
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

        // Modify the clean file (Modified)
        let mut file = File::create(&clean_path)?;
        writeln!(file, "Modified content")?;

        // Scan
        let git_map = get_repo_statuses(root);
        let node = scan_dir(root, 2, &git_map)?;

        // Verify "new_file.txt" is New
        let new_node = node
            .children
            .iter()
            .find(|c| c.name == "new_file.txt")
            .unwrap();
        assert_eq!(new_node.git_status, Some(GitStatus::New));

        // Verify "clean_file.txt" is Modified
        let mod_node = node
            .children
            .iter()
            .find(|c| c.name == "clean_file.txt")
            .unwrap();
        assert_eq!(mod_node.git_status, Some(GitStatus::Modified));

        Ok(())
    }

    #[test]
    fn test_get_view_root_has_parent() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();

        // Create subdir structure: root/child/grandchild
        let child = root.join("child");
        fs::create_dir(&child)?;
        let grandchild = child.join("grandchild");
        fs::create_dir(&grandchild)?;

        let git_map = HashMap::new();

        // Test 1: Root view (no parent)
        // tempdir creates a dir in /tmp, so it DOES have a parent (/tmp).
        // scan_dir uses absolute paths.
        // We need to be careful. tempdir path usually has parents.

        let node = get_view_root(&child, 1, &git_map)?;

        // Should have ".." pointing to root
        let parent_node = node.children.iter().find(|c| c.name == "..");
        assert!(parent_node.is_some());
        assert_eq!(parent_node.unwrap().path, root);

        Ok(())
    }
}
