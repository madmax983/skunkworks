use anyhow::{Context, Result};

use git2::{Oid, Repository};
use std::path::Path;

pub struct CommitInfo {
    pub id: String,
    pub message: String,
}

pub fn open_repo<P: AsRef<Path>>(path: P) -> Result<Repository> {
    Repository::open(path).context("Failed to open repository")
}

pub fn list_commits(repo: &Repository, limit: usize) -> Result<Vec<CommitInfo>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    let mut commits = Vec::new();
    for id in revwalk.take(limit) {
        let id = id?;
        let commit = repo.find_commit(id)?;

        commits.push(CommitInfo {
            id: id.to_string(),
            message: commit.summary().unwrap_or("").to_string(),
        });
    }
    Ok(commits)
}

pub fn get_file_content(repo: &Repository, commit_id: &str, filepath: &str) -> Result<String> {
    let oid = Oid::from_str(commit_id)?;
    let commit = repo.find_commit(oid)?;
    let tree = commit.tree()?;

    let entry = tree.get_path(Path::new(filepath));

    match entry {
        Ok(entry) => {
            let object = entry.to_object(repo)?;
            if let Some(blob) = object.as_blob() {
                let content = std::str::from_utf8(blob.content())
                    .unwrap_or("<Binary content>")
                    .to_string();
                Ok(content)
            } else {
                Ok("<Not a file>".to_string())
            }
        }
        Err(_) => Ok(format!("<File not found in this commit: {}>", filepath)),
    }
}
