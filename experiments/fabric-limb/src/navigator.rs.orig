use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub depth: usize,
}

pub struct FileExplorer {
    pub root: PathBuf,
    pub files: Vec<FileEntry>,
    pub selected_idx: usize,
}

impl FileExplorer {
    pub fn new(root: PathBuf) -> Result<Self> {
        Ok(Self {
            root,
            files: vec![],
            selected_idx: 0,
        })
    }

    pub fn scan(&mut self) -> Result<()> {
        self.files.clear();
        for entry in WalkDir::new(&self.root).sort_by_file_name() {
            let entry = entry?;
            let path = entry.path();
            // Calculate depth relative to root
            let depth = path.strip_prefix(&self.root)?.components().count();

            self.files.push(FileEntry {
                path: path.to_path_buf(),
                is_dir: entry.file_type().is_dir(),
                depth,
            });
        }
        Ok(())
    }

    pub fn move_cursor(&mut self, delta: i32) {
        if self.files.is_empty() {
            return;
        }
        let len = self.files.len() as i32;
        let new_idx = (self.selected_idx as i32 + delta).clamp(0, len - 1);
        self.selected_idx = new_idx as usize;
    }

    pub fn selected_file(&self) -> Option<&FileEntry> {
        self.files.get(self.selected_idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_traversal() -> Result<()> {
        // Create a temp dir structure
        let temp_dir = tempfile::tempdir()?;
        let root = temp_dir.path();

        fs::write(root.join("a.txt"), "content")?;
        fs::create_dir(root.join("sub"))?;
        fs::write(root.join("sub/b.rs"), "content")?;

        let mut explorer = FileExplorer::new(root.to_path_buf())?;
        explorer.scan()?;

        assert!(!explorer.files.is_empty(), "Explorer found no files");
        assert!(explorer.files.iter().any(|f| f.path.ends_with("a.txt")));
        assert!(explorer.files.iter().any(|f| f.path.ends_with("b.rs")));

        Ok(())
    }

    #[test]
    fn test_cursor_movement() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let root = temp_dir.path();
        fs::write(root.join("a.txt"), "1")?;
        fs::write(root.join("b.txt"), "2")?;

        let mut explorer = FileExplorer::new(root.to_path_buf())?;
        explorer.scan()?;
        // Should have root, a.txt, b.txt

        explorer.move_cursor(1);
        assert_eq!(explorer.selected_idx, 1);

        explorer.move_cursor(-1);
        assert_eq!(explorer.selected_idx, 0);

        explorer.move_cursor(-5); // Clamp
        assert_eq!(explorer.selected_idx, 0);

        Ok(())
    }
}
