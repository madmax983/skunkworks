use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use git2::{Delta, DiffOptions, Oid, Repository};

#[derive(Debug, Clone)]
pub struct Ghost {
    pub path: String,
    pub deleted_at: DateTime<Utc>,
    pub blob_id: Oid,
    pub size: usize,
    pub commit_message: String,
    pub author: String,
    pub commit_hash: String,
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
        // If no parent, it's the initial commit, so nothing was deleted *in* this commit relative to history.
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
                let path = old_file
                    .path()
                    .and_then(|p| p.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                let blob_id = old_file.id();
                let size = old_file.size() as usize;

                // If we already have a ghost for this path, skip it.
                // We want the *most recent* deletion of a path.
                // Since we iterate from HEAD backwards, the first time we see a deletion for path X,
                // it is the most recent one.
                if ghosts.iter().any(|g: &Ghost| g.path == path) {
                    continue;
                }

                let time =
                    DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0).unwrap_or_default();

                ghosts.push(Ghost {
                    path,
                    deleted_at: time,
                    blob_id,
                    size,
                    commit_message: commit.summary().unwrap_or("").to_string(),
                    author: commit.author().name().unwrap_or("Unknown").to_string(),
                    commit_hash: oid.to_string(),
                });
            }
        }
    }

    Ok(ghosts)
}

pub fn fetch_ectoplasm(repo: &Repository, ghost: &Ghost) -> Result<String> {
    if ghost.blob_id.is_zero() {
        return Ok("<VOID>".to_string());
    }
    let blob = repo.find_blob(ghost.blob_id)?;
    // Use lossy conversion to handle potential binary data or weird encodings
    let content = String::from_utf8_lossy(blob.content());
    Ok(content.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use tempfile::Builder;

    #[test]
    fn test_ghost_scan() -> Result<()> {
        let td = Builder::new().prefix("git-ghost-test").tempdir()?;
        let repo = Repository::init(td.path())?;

        // Commit 1: Add file
        let path = td.path().join("ghost.txt");
        {
            let mut f = File::create(&path)?;
            writeln!(f, "I am alive")?;
        }

        let mut index = repo.index()?;
        index.add_path(Path::new("ghost.txt"))?;
        index.write()?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        let sig = repo.signature()?;
        let parent_commit_oid =
            repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;
        let parent_commit = repo.find_commit(parent_commit_oid)?;

        // Commit 2: Delete file
        std::fs::remove_file(&path)?;
        index.remove_path(Path::new("ghost.txt"))?;
        index.write()?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Delete file",
            &tree,
            &[&parent_commit],
        )?;

        // Scan
        let ghosts = scan_graveyard(td.path().to_str().unwrap(), 10)?;
        assert_eq!(ghosts.len(), 1);
        assert_eq!(ghosts[0].path, "ghost.txt");

        // Fetch content
        let content = fetch_ectoplasm(&repo, &ghosts[0])?;
        assert!(content.contains("I am alive"));

        Ok(())
    }
}
