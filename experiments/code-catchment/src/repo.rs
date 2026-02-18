use git2::{Repository, TreeWalkResult, TreeWalkMode};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct RepoScanner {
    base_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub line_count: usize,
    pub grid_pos: (usize, usize),
}

impl RepoScanner {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, git2::Error> {
        Ok(Self {
            base_path: path.as_ref().to_path_buf(),
        })
    }

    pub fn scan_files(&self) -> Result<Vec<FileInfo>, git2::Error> {
        let repo = Repository::open(&self.base_path)?;
        let head = repo.head()?;
        let tree = head.peel_to_tree()?;
        let mut raw_files: Vec<(PathBuf, usize)> = Vec::new();

        tree.walk(TreeWalkMode::PreOrder, |root, entry| {
            if let Some(name) = entry.name() {
                if entry.kind() == Some(git2::ObjectType::Blob) {
                    let relative_path = PathBuf::from(root).join(name);
                    let full_path = self.base_path.join(&relative_path);

                    // Count lines from filesystem (simple approach)
                    // If file is binary or large, this might be slow/wrong, but okay for experiment.
                    if let Ok(lc) = count_lines(&full_path) {
                         raw_files.push((relative_path, lc));
                    }
                }
            }
            TreeWalkResult::Ok
        })?;

        // Sort files to ensure deterministic grid mapping
        raw_files.sort_by(|a, b| a.0.cmp(&b.0));

        let count = raw_files.len();
        if count == 0 {
            return Ok(Vec::new());
        }

        let grid_width = (count as f64).sqrt().ceil() as usize;

        let mut file_infos = Vec::new();
        for (i, (path, line_count)) in raw_files.into_iter().enumerate() {
            let x = i % grid_width;
            let y = i / grid_width;
            file_infos.push(FileInfo {
                path,
                line_count,
                grid_pos: (x, y),
            });
        }

        Ok(file_infos)
    }
}

fn count_lines(path: &Path) -> std::io::Result<usize> {
    // Note: This reads from working directory.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // Simple line count. Might fail on binaries, ignore errors by filtering.
    // Also, handle non-utf8?
    // lines() fails on non-utf8.
    // Use read_line or byte count?
    // Let's just use lines().count() and catch error.
    let count = reader.lines().count();
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;

    #[test]
    fn test_repo_scanner() {
        let temp_dir = std::env::temp_dir().join("code_catchment_test_repo");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        // Init git repo
        Command::new("git").arg("init").current_dir(&temp_dir).status().unwrap();

        // Add a file
        let file_path = temp_dir.join("test.rs");
        fs::write(&file_path, "fn main() {\n    println!(\"Hello\");\n}\n").unwrap();

        // Config git user (needed for commit)
        Command::new("git").args(&["config", "user.email", "you@example.com"]).current_dir(&temp_dir).status().unwrap();
        Command::new("git").args(&["config", "user.name", "Your Name"]).current_dir(&temp_dir).status().unwrap();

        // Commit
        Command::new("git").args(&["add", "."]).current_dir(&temp_dir).status().unwrap();
        Command::new("git").args(&["commit", "-m", "Initial commit"]).current_dir(&temp_dir).status().unwrap();

        // Test scanner
        let scanner = RepoScanner::new(&temp_dir).unwrap();
        let files = scanner.scan_files().unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path.file_name().unwrap(), "test.rs");
        assert_eq!(files[0].line_count, 3);
        assert_eq!(files[0].grid_pos, (0, 0));

        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
