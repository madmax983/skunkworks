use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct FileMetric {
    pub size: u64,
    pub lines: usize,
    pub complexity: f32, // Average indentation depth
}

pub struct Scanner;

impl Scanner {
    pub fn scan_file<P: AsRef<Path>>(path: P) -> io::Result<FileMetric> {
        let metadata = fs::metadata(&path)?;
        let size = metadata.len();

        let file = fs::File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut lines = 0;
        let mut total_indentation = 0;

        for line in reader.lines() {
            let line = line?;
            lines += 1;

            // Calculate indentation
            let trimmed = line.trim_start();
            if !trimmed.is_empty() {
                let indent_spaces = line.len() - trimmed.len();
                total_indentation += indent_spaces;
            }
        }

        let complexity = if lines > 0 {
            (total_indentation as f32) / 4.0 / (lines as f32)
        } else {
            0.0
        };

        Ok(FileMetric {
            size,
            lines,
            complexity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_scanner_metrics() {
        // Create a temporary file with known content
        let content =
            "fn main() {\n    let x = 1;\n    if x > 0 {\n        println!(\"Hello\");\n    }\n}";
        // Lines: 6
        // Size: len(content)
        // Complexity:
        // L1: 0
        // L2: 4
        // L3: 4
        // L4: 8
        // L5: 4
        // L6: 0
        // Total indent: 20 spaces. If 4 spaces = 1 level, then 5 levels total. Avg = 5/6 = 0.833...

        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        write!(file, "{}", content).expect("Failed to write to temp file");

        let metric = Scanner::scan_file(file.path()).expect("Failed to scan file");

        assert_eq!(metric.lines, 6);
        assert_eq!(metric.size, content.len() as u64);
        // We'll allow some float error
        assert!((metric.complexity - (20.0 / 4.0 / 6.0)).abs() < 0.001);
    }
}
