use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use git2::{Repository, Sort};

#[derive(Debug, Clone)]
pub struct CommitData {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub time: DateTime<Utc>,
    pub insertions: usize,
    pub deletions: usize,
    pub files_changed: usize,
}

pub fn load_recent_commits(repo_path: &str, limit: usize) -> Result<Vec<CommitData>> {
    let repo = Repository::discover(repo_path).context("Failed to open git repository")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TIME)?;

    let mut commits = Vec::new();
    let mut count = 0;

    for oid in revwalk {
        if count >= limit {
            break;
        }
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        let message = commit.message().unwrap_or("").to_string();
        let author = commit.author().name().unwrap_or("Unknown").to_string();
        let time = DateTime::from_timestamp(commit.time().seconds(), 0)
            .unwrap_or_default()
            .with_timezone(&Utc);

        // Diff stats
        let mut insertions = 0;
        let mut deletions = 0;
        let mut files_changed = 0;

        // Try to get parent for diff
        // If merge commit, just diff against first parent for simplicity
        if commit.parent_count() > 0 {
            if let Ok(parent) = commit.parent(0) {
                let tree = commit.tree()?;
                let parent_tree = parent.tree()?;
                let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
                let stats = diff.stats()?;
                insertions = stats.insertions();
                deletions = stats.deletions();
                files_changed = stats.files_changed();
            }
        } else {
            // Initial commit
            if let Ok(tree) = commit.tree() {
                 let diff = repo.diff_tree_to_tree(None, Some(&tree), None)?;
                 let stats = diff.stats()?;
                 insertions = stats.insertions();
                 deletions = stats.deletions();
                 files_changed = stats.files_changed();
            }
        }

        commits.push(CommitData {
            hash: oid.to_string(),
            message,
            author,
            time,
            insertions,
            deletions,
            files_changed,
        });
        count += 1;
    }

    Ok(commits)
}
