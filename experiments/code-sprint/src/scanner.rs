use anyhow::{Context, Result};
use rand::Rng;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Recursively scan the directory for valid .rs files.
pub fn scan_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Load a random snippet of code from the file.
/// Returns a String containing 5-15 lines of code, with common indentation removed.
pub fn load_snippet(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path).context("Failed to read file")?;
    let lines: Vec<&str> = content.lines().collect();

    if lines.len() < 10 {
        // If file is too short, just return the whole thing
        return Ok(trim_snippet(&lines).join("\n"));
    }

    let mut rng = rand::thread_rng();
    // Try a few times to find a good chunk
    for _ in 0..10 {
        let max_start = lines.len().saturating_sub(10);
        let start_index = rng.gen_range(0..=max_start);
        let length = rng.gen_range(10..=15).min(lines.len() - start_index);

        let chunk = &lines[start_index..start_index + length];

        // Filter out bad chunks (mostly empty or just comments)
        if is_interesting_chunk(chunk) {
            return Ok(trim_snippet(chunk).join("\n"));
        }
    }

    // Fallback: just take the first 10 lines
    let length = 10.min(lines.len());
    Ok(trim_snippet(&lines[0..length]).join("\n"))
}

fn is_interesting_chunk(lines: &[&str]) -> bool {
    let non_empty = lines.iter().filter(|l| !l.trim().is_empty()).count();
    let comments = lines.iter().filter(|l| l.trim().starts_with("//")).count();

    // We want at least 50% code (non-empty, non-comment)
    non_empty > lines.len() / 2 && comments < lines.len() / 3
}

fn trim_snippet(lines: &[&str]) -> Vec<String> {
    if lines.is_empty() {
        return Vec::new();
    }

    // Find minimum indentation
    let min_indent = lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0);

    lines
        .iter()
        .map(|l| {
            if l.trim().is_empty() {
                String::new()
            } else {
                // Safely slice string by char indices
                l.chars().skip(min_indent).collect()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_snippet() {
        let input = vec!["    fn main() {", "        println!(\"Hello\");", "    }"];
        let output = trim_snippet(&input);
        assert_eq!(output[0], "fn main() {");
        assert_eq!(output[1], "    println!(\"Hello\");"); // Still indented relative to main
        assert_eq!(output[2], "}");
    }
}
