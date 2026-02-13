//! # Git Associates 🤝
//!
//! A friendly, high-level wrapper around `git2` for analyzing repository history, diffs, and file changes.
//!
//! This crate simplifies common git operations needed for visualization tools or analysis scripts,
//! abstracting away the complexities of `git2`'s low-level API.
//!
//! ## Features
//!
//! - **History Traversal**: Easily fetch commit logs with metadata.
//! - **Diff Analysis**: Get detailed stats on insertions, deletions, and file modifications.
//! - **Working Directory**: Diff the current working directory against `HEAD`.
//! - **Hunk Extraction**: Parse diffs into structured hunks and lines.
//!
//! ## Example
//!
//! ```no_run
//! use git_associates::GitModel;
//!
//! fn main() -> anyhow::Result<()> {
//!     // Open the repository in the current directory
//!     let model = GitModel::open(".")?;
//!
//!     // Fetch the last 10 commits
//!     let history = model.history(10)?;
//!
//!     for commit in history {
//!         println!("{} - {}", commit.short_hash, commit.message);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod model;

use anyhow::{Context, Result};
use chrono::{TimeZone, Utc};
use git2::{DiffFlags, Repository, Sort};
pub use model::{Commit, CommitStats, DiffStats, FileChange, Hunk, LineChange};
use std::path::Path;

/// A wrapper around a Git repository that provides high-level analysis methods.
///
/// `GitModel` is the entry point for the `git-associates` library. It wraps a standard `git2::Repository`
/// and provides methods tailored for data analysis, visualization, and semantic understanding of
/// repository history.
///
/// It prioritizes usability over raw performance or exhaustiveness, making it ideal for tools
/// that need to "understand" the codebase evolution.
///
/// # Examples
///
/// ```no_run
/// use git_associates::GitModel;
///
/// let model = GitModel::open(".").unwrap();
/// let changes = model.diff_workdir().unwrap();
/// println!("Modified files: {}", changes.files.len());
/// ```
pub struct GitModel {
    repo: Repository,
}

impl GitModel {
    /// Opens a git repository at the specified path.
    ///
    /// The path can be the root of the repository or any subdirectory within it.
    /// This function uses `git2::Repository::discover` to find the git directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the repository or a subdirectory.
    ///
    /// # Errors
    ///
    /// Returns an error if the repository cannot be found or opened.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::discover(path).context("Failed to discover git repository")?;
        Ok(Self { repo })
    }

    /// Retrieves the commit history with basic metadata.
    ///
    /// This method fetches the most recent commits up to the specified limit.
    /// It does *not* include detailed diff statistics or file changes, making it faster
    /// than [`history_with_diffs`](Self::history_with_diffs).
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of commits to retrieve.
    pub fn history(&self, limit: usize) -> Result<Vec<Commit>> {
        self.history_internal(limit, false)
    }

    /// Retrieves the commit history including file diff statistics.
    ///
    /// In addition to basic metadata, this method computes the diff for each commit against its parent,
    /// populating the `stats` and `files` fields of the [`Commit`] struct.
    ///
    /// # Performance
    ///
    /// This operation is more expensive than [`history`](Self::history) because it involves
    /// computing diffs for every commit.
    pub fn history_with_diffs(&self, limit: usize) -> Result<Vec<Commit>> {
        self.history_internal(limit, true)
    }

    fn history_internal(&self, limit: usize, compute_diffs: bool) -> Result<Vec<Commit>> {
        let mut revwalk = self.repo.revwalk().context("Failed to create revwalker")?;
        revwalk.set_sorting(Sort::TIME)?;
        revwalk.push_head()?;

        let mut commits = Vec::new();

        for oid in revwalk.take(limit) {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            let hash = oid.to_string();
            let short_hash = hash.chars().take(7).collect();
            let author = commit.author().name().unwrap_or("Unknown").to_string();
            let message = commit.message().unwrap_or("").trim().to_string();
            let timestamp = Utc.timestamp_opt(commit.time().seconds(), 0).unwrap();

            let parents: Vec<String> = commit.parents().map(|p| p.id().to_string()).collect();

            // Stats
            let (stats, files) = if compute_diffs {
                match self.get_commit_diff(&commit, true) {
                    Ok((s, f)) => (Some(s), f),
                    Err(_) => (None, Vec::new()),
                }
            } else {
                (None, Vec::new())
            };

            commits.push(Commit {
                hash,
                short_hash,
                author,
                message,
                timestamp,
                parents,
                stats,
                files,
            });
        }

        Ok(commits)
    }

    fn get_commit_diff(
        &self,
        commit: &git2::Commit,
        include_hunks: bool,
    ) -> Result<(CommitStats, Vec<FileChange>)> {
        let tree = commit.tree()?;
        let parent = commit.parent(0).ok();
        let parent_tree = parent.as_ref().and_then(|p| p.tree().ok());

        let diff = self
            .repo
            .diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;

        let (total_insertions, total_deletions, files) =
            self.process_diff_internal(&diff, include_hunks)?;

        Ok((
            CommitStats {
                insertions: total_insertions,
                deletions: total_deletions,
                files_changed: files.len(),
            },
            files,
        ))
    }

    /// Computes the diff between the working directory and the HEAD commit.
    ///
    /// This is useful for checking uncommitted changes (both staged and unstaged).
    /// Untracked files are included in the diff.
    ///
    /// # Returns
    ///
    /// A [`DiffStats`] object containing details about modified, added, and removed files.
    pub fn diff_workdir(&self) -> Result<DiffStats> {
        let mut diff_opts = git2::DiffOptions::new();
        diff_opts.include_untracked(true);

        let head = self.repo.head().ok();
        let tree = head.as_ref().and_then(|h| h.peel_to_tree().ok());

        let diff = self
            .repo
            .diff_tree_to_workdir_with_index(tree.as_ref(), Some(&mut diff_opts))?;

        // Always include hunks for workdir diff
        let (total_added, total_removed, files) = self.process_diff_internal(&diff, true)?;

        Ok(DiffStats {
            files,
            total_added,
            total_removed,
        })
    }

    fn process_diff_internal(
        &self,
        diff: &git2::Diff,
        include_hunks: bool,
    ) -> Result<(usize, usize, Vec<FileChange>)> {
        let mut files = Vec::new();
        let mut total_insertions = 0;
        let mut total_deletions = 0;

        for i in 0..diff.deltas().len() {
            let Ok(Some(patch)) = git2::Patch::from_diff(diff, i) else {
                continue;
            };

            let delta = patch.delta();
            let path = delta
                .new_file()
                .path()
                .or(delta.old_file().path())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let extension = Path::new(&path)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();

            let stats = patch.line_stats().unwrap_or((0, 0, 0));
            let insertions = stats.1;
            let deletions = stats.2;

            total_insertions += insertions;
            total_deletions += deletions;

            let hunks = if include_hunks {
                Self::extract_hunks(&patch)
            } else {
                Vec::new()
            };

            files.push(FileChange {
                path,
                extension,
                insertions,
                deletions,
                is_binary: delta.flags().contains(DiffFlags::BINARY),
                hunks,
            });
        }

        Ok((total_insertions, total_deletions, files))
    }

    fn extract_hunks(patch: &git2::Patch) -> Vec<Hunk> {
        let mut hunks = Vec::new();
        for h_idx in 0..patch.num_hunks() {
            let Ok((hunk_info, lines_count)) = patch.hunk(h_idx) else {
                continue;
            };
            let mut hunk_lines = Vec::new();
            for l_idx in 0..lines_count {
                if let Ok(line) = patch.line_in_hunk(h_idx, l_idx) {
                    let content = std::str::from_utf8(line.content())
                        .unwrap_or("")
                        .to_string();
                    match line.origin() {
                        '+' => hunk_lines.push(LineChange::Added(content)),
                        '-' => hunk_lines.push(LineChange::Removed(content)),
                        ' ' => hunk_lines.push(LineChange::Context(content)),
                        _ => {}
                    }
                }
            }
            hunks.push(Hunk {
                header: std::str::from_utf8(hunk_info.header())
                    .unwrap_or("")
                    .to_string(),
                lines: hunk_lines,
            });
        }
        hunks
    }
}
