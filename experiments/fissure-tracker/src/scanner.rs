use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone, Default)]
pub struct ScanResult {
    pub path: PathBuf,
    pub unwrap_count: usize,
    pub expect_count: usize,
    pub panic_count: usize,
    pub todo_count: usize,
}

impl ScanResult {
    pub fn total_stress(&self) -> usize {
        self.unwrap_count + self.expect_count + self.panic_count * 5 + self.todo_count
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "." && s != "..")
        .unwrap_or(false)
}

fn is_target_dir(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s == "target" || s == ".git" || s == "node_modules")
        .unwrap_or(false)
}

pub fn scan_workspace(root: PathBuf) -> Vec<ScanResult> {
    let mut results = Vec::new();

    let walker = WalkDir::new(root).into_iter();
    for entry in walker.filter_entry(|e| (!is_hidden(e) || e.depth() == 0) && !is_target_dir(e)) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        // Only scan Rust files for now
        if path.extension().map_or(false, |ext| ext == "rs") {
            if let Ok(result) = scan_file(path.to_path_buf()) {
                if result.total_stress() > 0 {
                    results.push(result);
                }
            }
        }
    }
    results
}

fn scan_file(path: PathBuf) -> std::io::Result<ScanResult> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut result = ScanResult {
        path: path.clone(),
        ..Default::default()
    };

    for line in reader.lines() {
        let line = line?;
        // Simple string matching, not full parsing (good enough for visualization)
        if line.contains("unwrap()") {
            result.unwrap_count += 1;
        }
        if line.contains("expect(") {
            result.expect_count += 1;
        }
        if line.contains("panic!(") {
            result.panic_count += 1;
        }
        if line.contains("todo!(") || line.contains("unimplemented!(") {
            result.todo_count += 1;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_scan_counts() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("risky.rs");
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "fn main() {{").unwrap();
            writeln!(file, "    let x = Option::None.unwrap();").unwrap();
            writeln!(file, "    let y = Result::Err.expect(\"fail\");").unwrap();
            writeln!(file, "    panic!(\"boom\");").unwrap();
            writeln!(file, "    todo!(\"finish\");").unwrap();
            writeln!(file, "}}").unwrap();
        }

        let results = scan_workspace(dir.path().to_path_buf());
        assert_eq!(results.len(), 1);
        let res = &results[0];
        assert_eq!(res.unwrap_count, 1);
        assert_eq!(res.expect_count, 1);
        assert_eq!(res.panic_count, 1);
        assert_eq!(res.todo_count, 1);
        assert_eq!(res.total_stress(), 1 + 1 + 5 + 1);
    }
}
