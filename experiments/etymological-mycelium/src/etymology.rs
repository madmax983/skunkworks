use anyhow::{Context, Result};
use git2::{Oid, Repository};
use std::path::Path;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct LineNode {
    pub content: String,
    pub original_line_num: usize,
    pub id: String, // Unique ID based on content + original position
}

#[allow(dead_code)]
pub struct CommitLayer {
    pub oid: Oid,
    pub message: String,
    pub author: String,
    pub lines: Vec<LineNode>,
}

pub struct EtymologyRiver {
    pub layers: Vec<CommitLayer>,
}

impl EtymologyRiver {
    pub fn new(repo_path: &str, file_path: &str, limit: usize) -> Result<Self> {
        let repo = Repository::open(repo_path).context("Failed to open repo")?;
        let path = Path::new(file_path);

        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        let mut layers = Vec::new();
        let commits: Vec<Oid> = revwalk.take(limit).filter_map(Result::ok).collect();

        // Process in reverse (oldest to newest) to simulate growth
        // Or newest to oldest if we want to trace back?
        // Let's do newest to oldest (standard log) but maybe display left-to-right?
        // Let's stick to the order from revwalk (Newest -> Oldest) but maybe we want to grow from Oldest -> Newest?
        // Actually, revwalk gives Newest first. Let's reverse it to grow forward in time.

        let commits: Vec<Oid> = commits.into_iter().rev().collect();

        for oid in commits {
            let commit = repo.find_commit(oid)?;
            let tree = commit.tree()?;

            // Get file content
            let content = match tree.get_path(path) {
                Ok(entry) => {
                    let object = entry.to_object(&repo)?;
                    if let Some(blob) = object.as_blob() {
                        String::from_utf8_lossy(blob.content()).to_string()
                    } else {
                        String::new()
                    }
                }
                Err(_) => String::new(), // File might not exist in this commit
            };

            let lines: Vec<LineNode> = content
                .lines()
                .enumerate()
                .map(|(i, s)| LineNode {
                    content: s.to_string(),
                    original_line_num: i,
                    id: format!("{}-{}", oid, i),
                })
                .collect();

            layers.push(CommitLayer {
                oid,
                message: commit.message().unwrap_or("").trim().to_string(),
                author: commit.author().name().unwrap_or("Unknown").to_string(),
                lines,
            });
        }

        Ok(EtymologyRiver { layers })
    }
}
