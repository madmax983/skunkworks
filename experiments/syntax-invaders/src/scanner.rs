use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Recursively scans the directory for .rs files and extracts words.
pub fn scan_words(path: &Path) -> Result<Vec<String>> {
    let mut words = HashSet::new();
    if path.is_dir() {
        visit_dirs(path, &mut words)?;
    }
    Ok(words.into_iter().collect())
}

fn visit_dirs(dir: &Path, words: &mut HashSet<String>) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, words)?;
            } else if let Some(ext) = path.extension() {
                if ext == "rs" {
                    extract_from_file(&path, words)?;
                }
            }
        }
    }
    Ok(())
}

fn extract_from_file(path: &Path, words: &mut HashSet<String>) -> Result<()> {
    let content = fs::read_to_string(path)?;
    for word in content.split(|c: char| !c.is_alphanumeric()) {
        if word.len() > 3 {
            words.insert(word.to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_scan_words() -> Result<()> {
        let dir = Path::new("test_scan_data");
        if dir.exists() {
            fs::remove_dir_all(dir)?;
        }
        fs::create_dir(dir)?;

        let file_path = dir.join("test.rs");
        let mut file = File::create(&file_path)?;
        writeln!(file, "struct NovaScanner {{ score: u32 }}")?;

        let words = scan_words(dir)?;

        // Cleanup first to ensure it runs even if assert fails? No, standard test behavior.
        fs::remove_dir_all(dir)?;

        assert!(words.contains(&"struct".to_string()));
        assert!(words.contains(&"NovaScanner".to_string()));
        assert!(words.contains(&"score".to_string()));
        assert!(!words.contains(&"u32".to_string())); // len is 3, condition is > 3

        Ok(())
    }
}
