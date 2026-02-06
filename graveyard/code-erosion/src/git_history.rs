use chrono::NaiveDateTime;
use std::process::Command;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub date: NaiveDateTime,
    pub changes: Vec<FileChange>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub added: usize,
    pub deleted: usize,
}

pub fn load_history() -> Vec<Commit> {
    println!("Loading git history...");
    // git log --name-stat --reverse --date=iso
    // Format:
    // commit <hash>
    // Author: ...
    // Date:   2024-05-21 12:00:00 +0000
    //
    //     Message
    //
    // <added> <deleted> <path>

    // We can use a custom format to make it easier to parse.
    // --pretty=format:"COMMIT:%H|%ad" --date=iso-strict
    // Followed by --name-stat

    let output = Command::new("git")
        .args(&[
            "log",
            "--reverse",
            "--date=iso-strict",
            "--pretty=format:COMMIT:%H|%ad",
            "--numstat", // Use numstat for machine readable numbers
        ])
        .output()
        .expect("Failed to execute git log");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut commits = Vec::new();

    let mut current_commit: Option<Commit> = None;

    for line in stdout.lines() {
        if line.starts_with("COMMIT:") {
            if let Some(c) = current_commit.take() {
                commits.push(c);
            }

            let parts: Vec<&str> = line.trim_start_matches("COMMIT:").split('|').collect();
            if parts.len() >= 2 {
                let hash = parts[0].to_string();
                let date_str = parts[1];
                // 2024-10-23T15:21:05-07:00
                if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
                    current_commit = Some(Commit {
                        hash,
                        date: date.naive_utc(),
                        changes: Vec::new(),
                    });
                }
            }
        } else if let Some(ref mut commit) = current_commit {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                // <added> <deleted> <path>
                // Sometimes binary files show as - - path
                let added = parts[0].parse().unwrap_or(0);
                let deleted = parts[1].parse().unwrap_or(0);
                let path = parts[2].to_string();

                commit.changes.push(FileChange {
                    path,
                    added,
                    deleted,
                });
            }
        }
    }

    if let Some(c) = current_commit {
        commits.push(c);
    }

    println!("Loaded {} commits.", commits.len());
    commits
}
