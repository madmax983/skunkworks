use crate::git::Commit;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileStats {
    pub path: PathBuf,
    pub churn: usize,
    pub fixes: usize,
    pub last_modified: i64,
    pub haunt_score: f64,
    pub recent_messages: Vec<String>,
}

pub fn analyze_repository(commits: &[Commit]) -> Vec<FileStats> {
    let mut stats_map: HashMap<PathBuf, FileStats> = HashMap::new();

    for commit in commits {
        let is_fix = is_fix_commit(&commit.subject);

        for file_path in &commit.files {
            let entry = stats_map
                .entry(file_path.clone())
                .or_insert_with(|| FileStats {
                    path: file_path.clone(),
                    churn: 0,
                    fixes: 0,
                    last_modified: 0,
                    haunt_score: 0.0,
                    recent_messages: Vec::new(),
                });

            entry.churn += 1;
            if is_fix {
                entry.fixes += 1;
            }
            if commit.timestamp > entry.last_modified {
                entry.last_modified = commit.timestamp;
            }

            // Keep last 5 messages
            if entry.recent_messages.len() < 5 {
                entry.recent_messages.push(format!("[{}] {}: {}", commit.hash, commit.author, commit.subject));
            }
        }
    }

    let mut results: Vec<FileStats> = stats_map.into_values().collect();

    // Calculate score
    for stat in &mut results {
        // Score formula: Churn + (Fixes * 5)
        // Adjust weights as needed
        stat.haunt_score = (stat.churn as f64) + (stat.fixes as f64 * 5.0);
    }

    // Sort by haunt score descending
    results.sort_by(|a, b| b.haunt_score.partial_cmp(&a.haunt_score).unwrap());

    results
}

fn is_fix_commit(subject: &str) -> bool {
    let s = subject.to_lowercase();
    s.contains("fix")
        || s.contains("bug")
        || s.contains("panic")
        || s.contains("error")
        || s.contains("issue")
}
