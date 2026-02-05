use crate::decay::DecayLevel;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct CompostFile {
    pub path: PathBuf,
    pub relative_path: String,
    pub timestamp: i64,
    pub decay_level: DecayLevel,
}

pub struct CompostBin {
    pub files: Vec<CompostFile>,
}

impl CompostBin {
    pub fn scan(root: &Path) -> Result<Self> {
        // Load timestamps relative to the root
        let file_timestamps = load_git_timestamps(root).unwrap_or_default();
        let mut files = Vec::new();
        let now = chrono::Utc::now().timestamp();

        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                // Skip .git directory and target directory
                if path.to_string_lossy().contains("/.git/")
                    || path.to_string_lossy().contains("/target/")
                    || path.to_string_lossy().contains("/node_modules/")
                {
                    continue;
                }

                // Extension check - let's be broad but skip binaries
                if path.extension().is_none() {
                    continue;
                }

                let ext = path.extension().unwrap().to_string_lossy();
                // Common source and text formats
                if [
                    "rs", "md", "toml", "json", "js", "ts", "py", "html", "css", "txt", "yml",
                    "yaml", "sh", "c", "cpp", "h",
                ]
                .contains(&ext.as_ref())
                {
                    let relative = path
                        .strip_prefix(root)
                        .unwrap_or(path)
                        .to_string_lossy()
                        .to_string();

                    // Git paths are usually forward slash. On Windows this might differ.
                    // Convert backslashes to forward slashes for lookup if needed.
                    let lookup_key = relative.replace('\\', "/");

                    let timestamp = *file_timestamps.get(&lookup_key).unwrap_or(&now);

                    let age = now - timestamp;
                    let decay_level = DecayLevel::from_age(age);

                    files.push(CompostFile {
                        path: path.to_path_buf(),
                        relative_path: relative,
                        timestamp,
                        decay_level,
                    });
                }
            }
        }

        files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

        Ok(Self { files })
    }
}

fn load_git_timestamps(root: &Path) -> Result<HashMap<String, i64>> {
    // git log --name-only --format="COMMIT %at"
    // We process from newest to oldest. The first time we see a file, that's its latest modification.

    let output = Command::new("git")
        .args(["log", "--name-only", "--format=COMMIT %at"])
        .current_dir(root)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .context("Failed to spawn git command")?
        .stdout
        .context("Failed to open git stdout")?;

    let reader = BufReader::new(output);
    let mut timestamps = HashMap::new();
    let mut current_time = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("COMMIT ") {
            if let Some(ts_str) = line.strip_prefix("COMMIT ") {
                current_time = ts_str.parse().unwrap_or(0);
            }
        } else {
            // It's a file path
            if !timestamps.contains_key(line) {
                timestamps.insert(line.to_string(), current_time);
            }
        }
    }

    Ok(timestamps)
}
