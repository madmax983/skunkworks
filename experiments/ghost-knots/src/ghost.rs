use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use git2::{Delta, DiffOptions, Oid, Repository};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Ghost {
    pub path: String,
    pub deleted_at: DateTime<Utc>,
    pub blob_id: Oid,
    pub size: usize,
    pub commit_message: String,
    pub author: String,
    pub commit_hash: String,
    pub content_snippet: String,
}

pub fn scan_graveyard(repo_path: &str, limit: usize) -> Result<Vec<Ghost>> {
    let repo = Repository::discover(repo_path).context("Failed to discover git repo")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut ghosts = Vec::new();

    // Iterate through commits
    for oid_result in revwalk {
        if ghosts.len() >= limit {
            break;
        }
        let oid = oid_result?;
        let commit = repo.find_commit(oid)?;

        // Get parent to diff against
        let parent = if commit.parent_count() > 0 {
            commit.parent(0)?
        } else {
            continue;
        };

        let tree = commit.tree()?;
        let parent_tree = parent.tree()?;

        // Diff parent (old) -> current (new).
        let mut diff_opts = DiffOptions::new();
        diff_opts.include_typechange(true);
        let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_opts))?;

        for delta in diff.deltas() {
            if delta.status() == Delta::Deleted {
                let old_file = delta.old_file();
                let path = old_file.path().and_then(|p| p.to_str()).unwrap_or("unknown").to_string();
                let blob_id = old_file.id();
                let size = old_file.size() as usize;

                if ghosts.iter().any(|g: &Ghost| g.path == path) {
                   continue;
                }

                let time = DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0)
                    .unwrap_or_default();

                // Fetch content
                let content_snippet = if !blob_id.is_zero() {
                    match repo.find_blob(blob_id) {
                        Ok(blob) => {
                            let content = String::from_utf8_lossy(blob.content());
                            content.chars().take(50).collect()
                        },
                        Err(_) => "<VOID>".to_string(),
                    }
                } else {
                    "<VOID>".to_string()
                };

                ghosts.push(Ghost {
                    path,
                    deleted_at: time,
                    blob_id,
                    size,
                    commit_message: commit.summary().unwrap_or("").to_string(),
                    author: commit.author().name().unwrap_or("Unknown").to_string(),
                    commit_hash: oid.to_string(),
                    content_snippet,
                });
            }
        }
    }

    Ok(ghosts)
}

#[allow(dead_code)]
pub fn fetch_ectoplasm(repo: &Repository, ghost: &Ghost) -> Result<String> {
    if ghost.blob_id.is_zero() {
        return Ok("<VOID>".to_string());
    }
    let blob = repo.find_blob(ghost.blob_id)?;
    // Use lossy conversion to handle potential binary data or weird encodings
    let content = String::from_utf8_lossy(blob.content());
    Ok(content.to_string())
}
