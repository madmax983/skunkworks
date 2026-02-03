use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FileNode {
    #[allow(dead_code)]
    pub path: PathBuf,
    pub todo_count: usize,
    #[allow(dead_code)]
    pub line_count: usize,
}

pub fn scan_codebase(root: &Path) -> Result<Vec<FileNode>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            // Count TODOs
            let (todo_count, line_count) = analyze_file(path).unwrap_or((0, 0));

            // Only add if interesting (has code or todos)
            if line_count > 0 {
                files.push(FileNode {
                    path: path.to_path_buf(),
                    todo_count,
                    line_count,
                });
            }
        }
    }

    Ok(files)
}

fn analyze_file(path: &Path) -> Result<(usize, usize)> {
    let file = File::open(path).with_context(|| format!("Failed to open {:?}", path))?;
    let reader = BufReader::new(file);

    let mut todo_count = 0;
    let mut line_count = 0;

    for line in reader.lines() {
        let line = line?;
        line_count += 1;
        if line.contains("TODO") || line.contains("FIXME") {
            todo_count += 1;
        }
    }

    Ok((todo_count, line_count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_analyze_file_self() {
        let mut temp_path = std::env::temp_dir();
        temp_path.push("voronoi_ants_test.rs");

        let mut file = File::create(&temp_path).unwrap();
        writeln!(file, "fn main() {{").unwrap();
        writeln!(file, "    // TODO: Implement this").unwrap();
        writeln!(file, "    // FIXME: Broken").unwrap();
        writeln!(file, "}}").unwrap();

        let (todos, lines) = analyze_file(&temp_path).unwrap();

        // Cleanup
        std::fs::remove_file(&temp_path).unwrap();

        assert_eq!(todos, 2);
        assert_eq!(lines, 4);
    }
}
