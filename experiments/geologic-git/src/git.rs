use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct CommitEvent {
    pub hash: String,
    pub timestamp: i64,
    pub files: Vec<String>,
}

pub struct GitHistory {
    pub commits: Vec<CommitEvent>,
}

impl GitHistory {
    pub fn load() -> Result<Self> {
        let output = Command::new("git")
            .args([
                "log",
                "--name-only",
                "--pretty=format:COMMIT %H %at",
                "--reverse",
            ])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn git command")?
            .stdout
            .context("Failed to open git stdout")?;

        let reader = BufReader::new(output);
        let mut commits = Vec::new();
        let mut current_commit: Option<CommitEvent> = None;

        for line_res in reader.lines() {
            let line = line_res?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("COMMIT ") {
                if let Some(commit) = current_commit.take() {
                    commits.push(commit);
                }

                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    current_commit = Some(CommitEvent {
                        hash: parts[1].to_string(),
                        timestamp: parts[2].parse().unwrap_or(0),
                        files: Vec::new(),
                    });
                }
            } else if let Some(commit) = &mut current_commit {
                commit.files.push(line.to_string());
            }
        }

        if let Some(commit) = current_commit {
            commits.push(commit);
        }

        Ok(Self { commits })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_commits() {
        // Mocking git output is hard without dependency injection or traits.
        // For now, let's verify it works on the actual repo (or at least doesn't crash).
        // This is a "smoke test".
        let history = GitHistory::load();
        assert!(history.is_ok());
        let history = history.unwrap();
        assert!(!history.commits.is_empty());

        let first = &history.commits[0];
        println!("First commit: {:?}", first);
        assert!(!first.hash.is_empty());
    }
}
