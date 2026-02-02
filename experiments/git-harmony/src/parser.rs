use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct DiffStats {
    pub total_files: usize,
    pub total_added: usize,
    pub total_removed: usize,
}

#[derive(Debug, Clone)]
pub struct DiffState {
    pub files: Vec<FileDiff>,
    pub stats: DiffStats,
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub path: String,
    pub is_binary: bool,
    pub hunks: Vec<Hunk>,
    pub added: usize,
    pub removed: usize,
}

#[derive(Debug, Clone)]
pub struct Hunk {
    pub header: String,
    pub lines: Vec<LineChange>,
}

#[derive(Debug, Clone)]
pub enum LineChange {
    Context(String),
    Added(String),
    Removed(String),
}

pub fn get_diff() -> Result<DiffState> {
    // Run git diff HEAD. If it fails, maybe there is no HEAD (fresh repo), so try git diff --staged or just empty.
    let output = Command::new("git").args(&["diff", "HEAD"]).output();

    // If command fails, return empty diff
    if output.is_err() {
        return Ok(DiffState {
            files: vec![],
            stats: DiffStats::default(),
        });
    }

    let output = output.unwrap();
    let diff_str = String::from_utf8_lossy(&output.stdout);
    parse_diff(&diff_str)
}

pub fn parse_diff(diff_str: &str) -> Result<DiffState> {
    let mut files = Vec::new();
    let mut current_file: Option<FileDiff> = None;
    let mut current_hunk: Option<Hunk> = None;

    let mut stats = DiffStats::default();

    for line in diff_str.lines() {
        if line.starts_with("diff --git") {
            // Save previous file
            if let Some(mut file) = current_file.take() {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
                stats.total_added += file.added;
                stats.total_removed += file.removed;
                files.push(file);
            }
            stats.total_files += 1;

            // Start new file
            // Format: diff --git a/path/to/file b/path/to/file
            // We'll take the b/ path from later lines if possible, or just guess here.
            // Simplified:
            current_file = Some(FileDiff {
                path: "unknown".to_string(), // Will be updated by +++ line
                is_binary: false,
                hunks: Vec::new(),
                added: 0,
                removed: 0,
            });
        } else if line.starts_with("index ") {
            // ignore
        } else if line.starts_with("--- ") {
            // ignore
        } else if line.starts_with("+++ ") {
            if let Some(file) = current_file.as_mut() {
                let new_path = line.trim_start_matches("+++ b/");
                // If it didn't have b/, try matching a/ or just take the whole thing
                if new_path == line {
                    // Fallback
                    file.path = line.trim_start_matches("+++ ").to_string();
                } else {
                    file.path = new_path.to_string();
                }
            }
        } else if line.starts_with("@@ ") {
            // New hunk
            if let Some(file) = current_file.as_mut() {
                if let Some(hunk) = current_hunk.take() {
                    file.hunks.push(hunk);
                }
            }

            current_hunk = Some(Hunk {
                header: line.to_string(),
                lines: Vec::new(),
            });
        } else if let Some(hunk) = current_hunk.as_mut() {
            if line.starts_with('+') && !line.starts_with("+++") {
                hunk.lines.push(LineChange::Added(line[1..].to_string()));
                if let Some(file) = current_file.as_mut() {
                    file.added += 1;
                }
            } else if line.starts_with('-') && !line.starts_with("---") {
                hunk.lines.push(LineChange::Removed(line[1..].to_string()));
                if let Some(file) = current_file.as_mut() {
                    file.removed += 1;
                }
            } else {
                hunk.lines.push(LineChange::Context(line.to_string()));
            }
        }
    }

    // Flush last
    if let Some(mut file) = current_file.take() {
        if let Some(hunk) = current_hunk.take() {
            file.hunks.push(hunk);
        }
        stats.total_added += file.added;
        stats.total_removed += file.removed;
        files.push(file);
    }

    Ok(DiffState { files, stats })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_diff() {
        let diff = r#"diff --git a/test.txt b/test.txt
index e69de29..d95f3ad 100644
--- a/test.txt
+++ b/test.txt
@@ -1,2 +1,2 @@
-Hello
+Hello World
 World
"#;
        let state = parse_diff(diff).unwrap();
        assert_eq!(state.files.len(), 1);
        assert_eq!(state.files[0].path, "test.txt");
        assert_eq!(state.files[0].added, 1);
        assert_eq!(state.files[0].removed, 1);
        assert_eq!(state.files[0].hunks.len(), 1);
        match &state.files[0].hunks[0].lines[0] {
            LineChange::Removed(s) => assert_eq!(s, "Hello"),
            _ => panic!("Expected removed"),
        }
    }
}
