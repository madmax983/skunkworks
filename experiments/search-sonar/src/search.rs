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
                let ext_str = ext.to_string_lossy().to_lowercase();
                if matches!(ext_str.as_str(), "png" | "jpg" | "jpeg" | "gif" | "ico" | "pdf" | "bin" | "exe" | "lock") {
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

    #[test]
    fn test_ignores_hidden_files() {
        let dir = tempdir().unwrap();

        // Hidden file
        let hidden_file_path = dir.path().join(".hidden.txt");
        {
            let mut file = File::create(&hidden_file_path).unwrap();
            writeln!(file, "secret match").unwrap();
        }

        // File in hidden directory
        let hidden_dir = dir.path().join(".hidden_dir");
        std::fs::create_dir(&hidden_dir).unwrap();
        let file_in_hidden_dir = hidden_dir.join("visible.txt");
        {
            let mut file = File::create(&file_in_hidden_dir).unwrap();
            writeln!(file, "secret match").unwrap();
        }

        let rx = spawn_search_thread("secret".to_string(), dir.path().to_path_buf());
        let results: Vec<SearchResult> = rx.iter().collect();
        assert!(results.is_empty(), "Should ignore hidden files and directories");
    }

    #[test]
    fn test_ignores_infrastructure_dirs() {
        let dir = tempdir().unwrap();

        for folder in &["target", ".git", "node_modules"] {
            let infra_dir = dir.path().join(folder);
            std::fs::create_dir(&infra_dir).unwrap();
            let file_path = infra_dir.join("source.rs");
            {
                let mut file = File::create(&file_path).unwrap();
                writeln!(file, "infra match").unwrap();
            }
        }

        let rx = spawn_search_thread("infra".to_string(), dir.path().to_path_buf());
        let results: Vec<SearchResult> = rx.iter().collect();
        assert!(results.is_empty(), "Should ignore infrastructure directories");
    }

    #[test]
    fn test_ignores_binary_extensions() {
        let dir = tempdir().unwrap();

        // Standard binary extensions
        for ext in &["png", "exe", "pdf"] {
            let file_path = dir.path().join(format!("image.{}", ext));
            {
                let mut file = File::create(&file_path).unwrap();
                writeln!(file, "binary match").unwrap();
            }
        }

        let rx = spawn_search_thread("binary".to_string(), dir.path().to_path_buf());
        let results: Vec<SearchResult> = rx.iter().collect();
        assert!(results.is_empty(), "Should ignore binary extensions");
    }

    #[test]
    fn test_ignores_binary_extensions_case_insensitive() {
        let dir = tempdir().unwrap();

        // Uppercase extension - should be ignored if logic is case insensitive
        let file_path = dir.path().join("IMAGE.PNG");
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "case match").unwrap();
        }

        let rx = spawn_search_thread("case".to_string(), dir.path().to_path_buf());
        let results: Vec<SearchResult> = rx.iter().collect();
        assert!(results.is_empty(), "Should ignore uppercase binary extensions");
    }

    #[test]
    fn test_multiple_matches() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("multi.txt");
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "match one").unwrap();
            writeln!(file, "no hit").unwrap();
            writeln!(file, "match two").unwrap();
        }

        let rx = spawn_search_thread("match".to_string(), dir.path().to_path_buf());
        let mut results: Vec<SearchResult> = rx.iter().collect();

        // Sort by line number to ensure deterministic order check
        results.sort_by_key(|r| r.line_num);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].line_num, 1);
        assert_eq!(results[0].content, "match one");
        assert_eq!(results[1].line_num, 3);
        assert_eq!(results[1].content, "match two");
    }
}
