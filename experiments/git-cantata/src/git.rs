use anyhow::{Context, Result};
use git2::{Repository, Sort};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub extension: String,
    pub insertions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone)]
pub struct CommitData {
    pub hash: String,
    pub author: String,
    pub message: String,
    pub timestamp: i64,
    pub changes: Vec<FileChange>,
}

pub fn get_repo_history(path: &str, limit: usize) -> Result<Vec<CommitData>> {
    let repo = Repository::discover(path).context("Failed to open git repo")?;
    let mut walker = repo.revwalk().context("Failed to create revwalker")?;
    walker.set_sorting(Sort::TIME)?;
    walker.push_head()?;

    let mut commits = Vec::new();

    for oid in walker.take(limit) {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        let hash = oid.to_string();
        let author = commit.author().name().unwrap_or("Unknown").to_string();
        let message = commit.message().unwrap_or("").trim().to_string();
        let timestamp = commit.time().seconds();

        let mut changes = Vec::new();

        if let (Ok(tree), Ok(parent)) = (commit.tree(), commit.parent(0)) {
            if let Ok(parent_tree) = parent.tree() {
                if let Ok(diff) = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None) {
                    // Iterate over deltas to get file stats
                    for i in 0..diff.deltas().len() {
                        // Patch::from_diff returns Result<Option<Patch>>
                        if let Ok(Some(patch)) = git2::Patch::from_diff(&diff, i) {
                            let delta = patch.delta();
                            // Try new file path, fallback to old file path
                            let file_path = delta.new_file().path().or(delta.old_file().path());

                            if let Some(p) = file_path {
                                let path_str = p.to_string_lossy().to_string();
                                let extension = Path::new(&path_str)
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("")
                                    .to_string();

                                // line_stats returns Result<(usize, usize, usize)>
                                let stats = patch.line_stats().unwrap_or((0, 0, 0)); // context, additions, deletions

                                changes.push(FileChange {
                                    path: path_str,
                                    extension,
                                    insertions: stats.1,
                                    deletions: stats.2,
                                });
                            }
                        }
                    }
                }
            }
        }

        commits.push(CommitData {
            hash,
            author,
            message,
            timestamp,
            changes,
        });
    }

    // Playback history: Oldest -> Newest
    commits.reverse();

    Ok(commits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_repo_history() {
        // Use current repo
        let history = get_repo_history(".", 10).expect("Failed to get history");
        assert!(!history.is_empty());

        let last = &history[history.len() - 1]; // Newest commit
        println!("Last commit: {} by {}", last.hash, last.author);

        assert_eq!(last.hash.len(), 40);
    }
}
