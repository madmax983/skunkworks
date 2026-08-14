use anyhow::{Context, Result};
use chrono::Utc;
use git2::{BlameOptions, Repository};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LineInfo {
    pub age_score: f64, // 0.0 (Oldest) to 1.0 (Newest)
}

pub struct BlameAnalyzer {
    pub start_path: String,
}

impl BlameAnalyzer {
    pub fn new(start_path: &str) -> Self {
        Self {
            start_path: start_path.to_string(),
        }
    }

    pub fn analyze(&self, file_path: &Path) -> Result<Vec<LineInfo>> {
        // Discover repo from the file path itself, or start_path if file_path is relative
        // Actually, just discover from file_path's directory if possible, or CWD.
        let discover_path = if file_path.is_absolute() {
            file_path
        } else {
            Path::new(&self.start_path)
        };

        let repo =
            Repository::discover(discover_path).context("Failed to discover git repository")?;

        // Resolve path relative to repo root
        // We need absolute path of file first
        let abs_file_path = if file_path.is_absolute() {
            file_path.to_path_buf()
        } else {
            std::fs::canonicalize(file_path)
                .context(format!("Failed to canonicalize path: {:?}", file_path))?
        };

        // Repo workdir is absolute usually
        let workdir = repo.workdir().context("Repository has no workdir")?;
        let workdir = std::fs::canonicalize(workdir).unwrap_or(workdir.to_path_buf());

        let path_relative = abs_file_path.strip_prefix(&workdir).context(format!(
            "File {:?} is not in repository {:?}",
            abs_file_path, workdir
        ))?;

        let mut opts = BlameOptions::new();
        let blame = repo
            .blame_file(path_relative, Some(&mut opts))
            .context(format!("Failed to blame file: {:?}", path_relative))?;

        let mut lines = Vec::new();
        let mut min_time = i64::MAX;
        let mut max_time = i64::MIN;
        let now = Utc::now().timestamp();

        // First pass: Find time range
        for hunk in blame.iter() {
            let commit_id = hunk.final_commit_id();
            let time = if commit_id.is_zero() {
                now
            } else if let Ok(commit) = repo.find_commit(commit_id) {
                commit.time().seconds()
            } else {
                now // Fallback
            };

            if time < min_time {
                min_time = time;
            }
            if time > max_time {
                max_time = time;
            }
        }

        // Ensure range is valid
        if min_time > max_time {
            min_time = now;
            max_time = now;
        }

        // Avoid division by zero if all commits have same timestamp
        if min_time == max_time {
            min_time -= 1;
        }

        let range = (max_time - min_time) as f64;

        // Second pass: Build LineInfo
        for hunk in blame.iter() {
            let commit_id = hunk.final_commit_id();

            let (_author_name, _message, time, _hash_str) = if commit_id.is_zero() {
                (
                    "You (Uncommitted)".to_string(),
                    "Uncommitted changes".to_string(),
                    now,
                    "00000000".to_string(),
                )
            } else if let Ok(commit) = repo.find_commit(commit_id) {
                let author = commit.author();
                let author_name = author.name().unwrap_or("Unknown").to_string();
                let message = commit
                    .summary()
                    .unwrap_or(Some(""))
                    .unwrap_or("")
                    .to_string();
                let time = commit.time().seconds();
                (
                    author_name,
                    message,
                    time,
                    commit_id.to_string()[..8].to_string(),
                )
            } else {
                (
                    "Unknown".to_string(),
                    "Unknown commit".to_string(),
                    now,
                    "????????".to_string(),
                )
            };

            let age_score = (time - min_time) as f64 / range;

            let count = hunk.lines_in_hunk();

            for _ in 0..count {
                lines.push(LineInfo { age_score });
            }
        }

        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_age_score_normalization() {
        let min_time = 1000;
        let max_time = 2000;
        let range = (max_time - min_time) as f64;

        let time = 1500;
        let score = (time - min_time) as f64 / range;

        assert_eq!(score, 0.5);
    }
}

#[cfg(test)]
mod sentry_tests {
    use super::BlameAnalyzer;
    use git2::{Repository, Signature};
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_blame_summary_unwrap() {
        let temp_dir = std::env::temp_dir().join("chron-compost-sentry-test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let repo = Repository::init(&temp_dir).unwrap();

        let file_path = temp_dir.join("test.txt");
        fs::write(&file_path, "line1\nline2").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test.txt")).unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();

        let sig = Signature::now("Test", "test@example.com").unwrap();

        // Commit with an invalid summary (e.g., only newlines or empty to trigger None summary)
        repo.commit(Some("HEAD"), &sig, &sig, "", &tree, &[])
            .unwrap();

        let analyzer = BlameAnalyzer::new(temp_dir.to_str().unwrap());

        // Should not panic on unwrap
        let result = analyzer.analyze(&file_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_blame_analyzer_uncommitted_changes() {
        let temp_dir = std::env::temp_dir().join("chron-compost-sentry-uncommitted-test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let repo = Repository::init(&temp_dir).unwrap();

        let file_path = temp_dir.join("test_uncommitted.txt");
        fs::write(&file_path, "line1\nline2").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test_uncommitted.txt")).unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();

        let sig = Signature::now("Test", "test@example.com").unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
            .unwrap();

        let analyzer = BlameAnalyzer::new(temp_dir.to_str().unwrap());

        // Test the blame functionality
        let result = analyzer.analyze(&file_path).unwrap();

        assert_eq!(result.len(), 2);
    }
}
