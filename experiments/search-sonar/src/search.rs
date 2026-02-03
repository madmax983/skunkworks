use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub line_num: usize,
    pub content: String,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with('.') && s != "." && s != "..")
         .unwrap_or(false)
}

fn is_target_dir(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s == "target" || s == ".git" || s == "node_modules")
         .unwrap_or(false)
}

pub fn spawn_search_thread(query: String, root: PathBuf) -> Receiver<SearchResult> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
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
            // Simple check to avoid binary files (not perfect, but MVP)
             if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy();
                if matches!(ext_str.as_ref(), "png" | "jpg" | "jpeg" | "gif" | "ico" | "pdf" | "bin" | "exe" | "lock") {
                    continue;
                }
            }

            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for (i, line) in reader.lines().enumerate() {
                    if let Ok(line_content) = line {
                        if line_content.contains(&query) {
                            let result = SearchResult {
                                path: path.to_path_buf(),
                                line_num: i + 1,
                                content: line_content,
                            };
                            if tx.send(result).is_err() {
                                return; // Receiver dropped
                            }
                        }
                    }
                }
            }
        }
    });

    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_search() {
        let dir = tempdir().unwrap();
        println!("Temp dir: {:?}", dir.path());
        let file_path = dir.path().join("test_file.txt");
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "Hello world").unwrap();
            writeln!(file, "This is a test").unwrap();
            writeln!(file, "Searching for rust").unwrap();
            file.sync_all().unwrap();
        } // Drop file to ensure close

        let rx = spawn_search_thread("test".to_string(), dir.path().to_path_buf());

        let results: Vec<SearchResult> = rx.iter().collect();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_num, 2);
        assert!(results[0].content.contains("test"));
    }
}
