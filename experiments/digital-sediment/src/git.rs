use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use git2::{Oid, Repository, Tree};
use std::path::Path;

pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub author: String,
    pub time: DateTime<Utc>,
    pub timestamp: i64,
}

pub struct RepoHandler {
    repo: Repository,
}

impl RepoHandler {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path).context("Failed to open repository")?;
        Ok(Self { repo })
    }

    pub fn list_commits(&self, limit: usize) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        let mut commits = Vec::new();
        for id in revwalk.take(limit) {
            let id = id?;
            let commit = self.repo.find_commit(id)?;
            let time = commit.time();
            let timestamp = time.seconds();
            let datetime = Utc
                .timestamp_opt(timestamp, 0)
                .single()
                .unwrap_or(Utc::now());

            commits.push(CommitInfo {
                id: id.to_string(),
                message: commit.summary().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("Unknown").to_string(),
                time: datetime,
                timestamp,
            });
        }
        Ok(commits)
    }

    pub fn get_file_content(&self, commit_id: &str, filepath: &str) -> Result<String> {
        let oid = Oid::from_str(commit_id)?;
        let commit = self.repo.find_commit(oid)?;
        let tree = commit.tree()?;

        let entry = tree.get_path(Path::new(filepath));

        match entry {
            Ok(entry) => {
                let object = entry.to_object(&self.repo)?;
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
}
