use anyhow::Result;
use git_associates::{GitModel, Commit};
use std::collections::VecDeque;
use crate::mapping::GitEvent;

pub struct GitScanner {
    model: GitModel,
    commit_queue: VecDeque<Commit>,
    current_commit: Option<Commit>,
    current_file_idx: usize,
    history_limit: usize,
}

impl GitScanner {
    pub fn new(path: &str, limit: usize) -> Result<Self> {
        let model = GitModel::open(path)?;
        let mut scanner = Self {
            model,
            commit_queue: VecDeque::new(),
            current_commit: None,
            current_file_idx: 0,
            history_limit: limit,
        };
        scanner.fetch_history()?;
        Ok(scanner)
    }

    fn fetch_history(&mut self) -> Result<()> {
        // Fetch history with diffs
        // Note: reverse to play from past to present?
        // GitModel returns most recent first.
        // So we should reverse it.
        let commits = self.model.history_with_diffs(self.history_limit)?;
        for commit in commits.into_iter().rev() {
            self.commit_queue.push_back(commit);
        }
        Ok(())
    }

    pub fn next_event(&mut self) -> Option<GitEvent> {
        loop {
            // If we have a current commit, process its files
            if let Some(commit) = &self.current_commit {
                if self.current_file_idx < commit.files.len() {
                    let file = &commit.files[self.current_file_idx];
                    self.current_file_idx += 1;

                    return Some(GitEvent {
                        file_path: file.path.clone(),
                        insertions: file.insertions,
                        deletions: file.deletions,
                        author: commit.author.clone(),
                        timestamp: commit.timestamp.timestamp(),
                    });
                } else {
                    // Done with this commit
                    self.current_commit = None;
                }
            }

            // No current commit, fetch next from queue
            if let Some(commit) = self.commit_queue.pop_front() {
                self.current_commit = Some(commit);
                self.current_file_idx = 0;
            } else {
                // Queue empty
                return None;
            }
        }
    }
}
