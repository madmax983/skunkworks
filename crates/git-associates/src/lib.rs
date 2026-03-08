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

#[doc(hidden)]
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

        // Optimization: Pre-allocate capacity to prevent multiple heap reallocations
        // during iterative population of commits up to the known `limit`.
        let mut commits = Vec::with_capacity(limit);

        for oid in revwalk.take(limit) {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            let hash = oid.to_string();
            let short_hash = hash[..7].to_owned();
            let author = commit
                .author()
                .name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            let message = commit
                .message()
                .map(|s| s.trim().to_string())
                .unwrap_or_default();

            // chrono LocalResult::unwrap() panics if the timestamp is out of range.
            // Using `single().unwrap_or_else(...)` provides a safe fallback (UNIX epoch).
            let timestamp = Utc
                .timestamp_opt(commit.time().seconds(), 0)
                .single()
                .unwrap_or_else(|| Utc.timestamp_opt(0, 0).unwrap());

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

    /// Internal helper to iterate over a diff and extract file statistics and changes.
    ///
    /// This function walks through every delta in the diff (which corresponds to a changed file).
    /// For each delta, it creates a `Patch` to inspect the line-by-line changes.
    ///
    /// It aggregates total insertions and deletions and constructs a list of `FileChange` objects.
    fn process_diff_internal(
        &self,
        diff: &git2::Diff,
        include_hunks: bool,
    ) -> Result<(usize, usize, Vec<FileChange>)> {
        // Optimization: Pre-allocate capacity based on the number of deltas in the diff
        // to prevent multiple heap reallocations during iterative population.
        let mut files = Vec::with_capacity(diff.deltas().len());
        let mut total_insertions = 0;
        let mut total_deletions = 0;

        for i in 0..diff.deltas().len() {
            // A Patch object lets us examine the hunks and lines of a delta
            let Ok(Some(patch)) = git2::Patch::from_diff(diff, i) else {
                continue;
            };

            let delta = patch.delta();
            // Try to get the new path, fallback to old path (e.g., for deletions)
            let path = delta
                .new_file()
                .path()
                .or(delta.old_file().path())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let extension = Path::new(&path)
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_string())
                .unwrap_or_default();

            // line_stats returns (context, insertions, deletions)
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

    /// Extracts detailed hunk information from a patch.
    ///
    /// A Hunk is a contiguous block of changes in a file. This function iterates
    /// through all hunks and their lines, classifying them as Added, Removed, or Context.
    fn extract_hunks(patch: &git2::Patch) -> Vec<Hunk> {
        // Optimization: Pre-allocate capacity based on the total number of hunks
        // to prevent multiple heap reallocations.
        let mut hunks = Vec::with_capacity(patch.num_hunks());
        for h_idx in 0..patch.num_hunks() {
            // Get the hunk header info and the number of lines in this hunk
            let Ok((hunk_info, lines_count)) = patch.hunk(h_idx) else {
                continue;
            };
            // Optimization: Pre-allocate capacity based on the total lines in the hunk
            // to prevent multiple heap reallocations.
            let mut hunk_lines = Vec::with_capacity(lines_count);
            for l_idx in 0..lines_count {
                if let Ok(line) = patch.line_in_hunk(h_idx, l_idx) {
                    let content = std::str::from_utf8(line.content())
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    // Origin character indicates the type of change:
                    // '+' = Addition, '-' = Deletion, ' ' = Context
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
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                lines: hunk_lines,
            });
        }
        hunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{Repository, Signature, Time};

    #[test]
    fn should_not_panic_on_invalid_commit_timestamp() {
        let temp_dir = std::env::temp_dir().join("git-associates-timestamp-test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        let repo = Repository::init(&temp_dir).unwrap();

        let mut index = repo.index().unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();

        // A timestamp that causes chrono's Utc.timestamp_opt to return None
        // 253402300800 seconds is the year 10000. We multiply by 2000 to exceed chrono's max.
        let time = Time::new(253402300800 * 2000, 0);
        let sig = Signature::new("Test Author", "test@example.com", &time).unwrap();

        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Invalid timestamp commit",
            &tree,
            &[],
        )
        .unwrap();

        let model = GitModel::open(temp_dir).unwrap();

        // This should NOT panic, but right now it will.
        let history = model.history_with_diffs(10).unwrap();
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_history_without_diffs() {
        let temp_dir = std::env::temp_dir().join("git-associates-history-test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        let repo = Repository::init(&temp_dir).unwrap();

        let mut index = repo.index().unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();

        let time = Time::new(1700000000, 0);
        let sig = Signature::new("Test Author", "test@example.com", &time).unwrap();

        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "History without diffs commit",
            &tree,
            &[],
        )
        .unwrap();

        let model = GitModel::open(temp_dir).unwrap();
        let history = model.history(10).unwrap();

        assert_eq!(history.len(), 1);
        let commit = &history[0];
        assert_eq!(commit.message, "History without diffs commit");
        assert!(
            commit.stats.is_none(),
            "Stats should be None when compute_diffs is false"
        );
        assert!(
            commit.files.is_empty(),
            "Files should be empty when compute_diffs is false"
        );
    }

    #[test]
    fn test_hunk_extraction_and_diffs() {
        let temp_dir = std::env::temp_dir().join("git-associates-hunk-test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();
        let repo = Repository::init(&temp_dir).unwrap();

        let file_path = temp_dir.join("context.txt");
        std::fs::write(&file_path, "line1\nline2\nline3\nline4\nline5\n").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(std::path::Path::new("context.txt")).unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();
        let sig = Signature::now("Test", "test@example.com").unwrap();

        let parent_commit = repo
            .commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();

        // Now modify to have context, addition, and deletion
        std::fs::write(
            &file_path,
            "line1\nmodified2\nline3\nline4\nnew line\nline5\n",
        )
        .unwrap();

        index.add_path(std::path::Path::new("context.txt")).unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();
        let parent = repo.find_commit(parent_commit).unwrap();

        repo.commit(Some("HEAD"), &sig, &sig, "Second commit", &tree, &[&parent])
            .unwrap();

        let model = GitModel::open(temp_dir).unwrap();
        let history = model.history_with_diffs(1).unwrap();

        let commit = &history[0];
        assert_eq!(commit.files.len(), 1);
        let file = &commit.files[0];
        assert_eq!(file.path, "context.txt");
        assert_eq!(file.insertions, 2);
        assert_eq!(file.deletions, 1);

        let hunk = &file.hunks[0];
        // We should have context lines, added, and removed lines
        let mut has_context = false;
        let mut has_added = false;
        let mut has_removed = false;

        for line in &hunk.lines {
            match line {
                LineChange::Context(_) => has_context = true,
                LineChange::Added(_) => has_added = true,
                LineChange::Removed(_) => has_removed = true,
            }
        }

        assert!(has_context);
        assert!(has_added);
        assert!(has_removed);
    }

    #[test]
    fn test_history_with_diffs_error_fallback() {
        // By deleting a tree object from git, we can force a failure in diff computation
        let temp_dir = std::env::temp_dir().join("git-associates-err-test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();
        let repo = Repository::init(&temp_dir).unwrap();

        let file_path = temp_dir.join("test.txt");
        std::fs::write(&file_path, "test").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(std::path::Path::new("test.txt")).unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();
        let sig = Signature::now("Test", "test@example.com").unwrap();

        let _commit_oid = repo
            .commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();

        // Remove the tree object to cause diff generation to fail
        let tree_str = tree.id().to_string();
        let tree_path = temp_dir
            .join(".git")
            .join("objects")
            .join(&tree_str[0..2])
            .join(&tree_str[2..]);
        std::fs::remove_file(tree_path).ok(); // ok if it fails but hopefully it doesn't

        // When we fetch the history, `commit.tree()` (called in `get_commit_diff`)
        // will fail because the tree object is missing. The error should be caught
        // and swallowed by the `Err(_)` arm in `history_internal`, falling back to
        // (None, Vec::new()).
        let model = GitModel::open(temp_dir).unwrap();
        let history = model
            .history_with_diffs(1)
            .expect("history_with_diffs should handle the diff error without propagating it");

        assert_eq!(history.len(), 1);
        assert!(history[0].stats.is_none());
        assert!(history[0].files.is_empty());
    }
}
