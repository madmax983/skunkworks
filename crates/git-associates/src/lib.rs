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
use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use git2::{DiffFlags, Repository, Sort};
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use git_associates::GitModel;
    ///
    /// // Open the repository in the current directory
    /// let model = GitModel::open(".").expect("Failed to open repository");
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use git_associates::GitModel;
    ///
    /// let model = GitModel::open(".").unwrap();
    /// // Get the last 5 commits
    /// let commits = model.history(5).unwrap();
    /// for commit in commits {
    ///     println!("Commit: {} by {}", commit.short_hash, commit.author);
    /// }
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use git_associates::GitModel;
    ///
    /// let model = GitModel::open(".").unwrap();
    /// let commits = model.history_with_diffs(5).unwrap();
    /// for commit in commits {
    ///     if let Some(stats) = &commit.stats {
    ///         println!(
    ///             "Commit {} changed {} files (+{}, -{})",
    ///             commit.short_hash, stats.files_changed, stats.insertions, stats.deletions
    ///         );
    ///     }
    /// }
    /// ```
    pub fn history_with_diffs(&self, limit: usize) -> Result<Vec<Commit>> {
        self.history_internal(limit, true)
    }

    fn history_internal(&self, limit: usize, compute_diffs: bool) -> Result<Vec<Commit>> {
        let mut revwalk = self.repo.revwalk().context("Failed to create revwalker")?;
        revwalk.set_sorting(Sort::TIME)?;
        revwalk.push_head()?;

        // Optimization: Pre-allocate capacity to prevent multiple heap reallocations
        // during iterative population of commits up to the known `limit`.
        // Capped to a safe bound (10_000) to prevent OOM / capacity overflow panics on unbounded user inputs.
        let mut commits = Vec::with_capacity(limit.min(10_000));

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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use git_associates::GitModel;
    ///
    /// let model = GitModel::open(".").unwrap();
    /// let stats = model.diff_workdir().unwrap();
    /// println!("You have {} uncommitted file changes.", stats.files.len());
    /// ```
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
        let num_hunks = patch.num_hunks();
        let mut hunks = Vec::with_capacity(num_hunks);

        for h_idx in 0..num_hunks {
            let Ok((hunk_info, lines_count)) = patch.hunk(h_idx) else {
                continue;
            };

            let mut lines = Vec::with_capacity(lines_count);
            for l_idx in 0..lines_count {
                if let Ok(line) = patch.line_in_hunk(h_idx, l_idx) {
                    if let Some(change) = Self::parse_line_change(&line) {
                        lines.push(change);
                    }
                }
            }

            hunks.push(Hunk {
                header: String::from_utf8_lossy(hunk_info.header()).into_owned(),
                lines,
            });
        }

        hunks
    }

    /// Helper to convert a git2::DiffLine into a LineChange based on origin.
    fn parse_line_change(line: &git2::DiffLine) -> Option<LineChange> {
        // Optimization: Do not allocate a String until we know the line origin is valid,
        // preventing unnecessary allocations for ignored line types (like file headers).
        match line.origin() {
            '+' => Some(LineChange::Added(
                String::from_utf8_lossy(line.content()).into_owned(),
            )),
            '-' => Some(LineChange::Removed(
                String::from_utf8_lossy(line.content()).into_owned(),
            )),
            ' ' => Some(LineChange::Context(
                String::from_utf8_lossy(line.content()).into_owned(),
            )),
            _ => None,
        }
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

    #[test]
    fn test_history_capacity_overflow_dos_prevention() {
        let temp_dir = std::env::temp_dir().join("git-associates-dos-test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        let repo = Repository::init(&temp_dir).unwrap();

        let mut index = repo.index().unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();

        let time = Time::new(1700000000, 0);
        let sig = Signature::new("Test Author", "test@example.com", &time).unwrap();

        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();

        let model = GitModel::open(temp_dir).unwrap();

        // This should not panic with an OOM abort.
        let history = model.history(usize::MAX).unwrap();
        assert_eq!(history.len(), 1);

        let history_with_diffs = model.history_with_diffs(usize::MAX).unwrap();
        assert_eq!(history_with_diffs.len(), 1);
    }

    #[test]
    fn test_process_diff_internal_without_hunks() {
        let temp_dir = std::env::temp_dir().join("git-associates-internal-hunks-test");
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

        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();

        std::fs::write(
            &file_path,
            "line1\nmodified2\nline3\nline4\nnew line\nline5\n",
        )
        .unwrap();

        let model = GitModel::open(temp_dir).unwrap();
        let diff = model
            .repo
            .diff_tree_to_workdir_with_index(Some(&tree), None)
            .unwrap();

        let (_, _, files) = model.process_diff_internal(&diff, false).unwrap();

        assert_eq!(files.len(), 1);
        assert!(files[0].hunks.is_empty());
    }
}
/// Represents a single commit in the git history.
///
/// This struct aggregates essential metadata about a commit, including its hash,
/// author, message, timestamp, and optional diff statistics.
///
/// # Examples
///
/// ```
/// use git_associates::Commit;
/// use chrono::Utc;
///
/// let commit = Commit {
///     hash: "d3b07384d113edec49eaa6238ad5ff00".to_string(),
///     short_hash: "d3b0738".to_string(),
///     author: "Jane Doe".to_string(),
///     message: "Fix bug in parser".to_string(),
///     timestamp: Utc::now(),
///     parents: vec!["a1b2c3d".to_string()],
///     stats: None,
///     files: vec![],
/// };
///
/// assert_eq!(commit.short_hash, "d3b0738");
/// ```
#[derive(Debug, Clone)]
pub struct Commit {
    /// The full SHA-1 hash of the commit.
    pub hash: String,
    /// The first 7 characters of the SHA-1 hash.
    pub short_hash: String,
    /// The name of the author (e.g., "John Doe").
    pub author: String,
    /// The commit message (subject and body).
    pub message: String,
    /// The timestamp of when the commit was created (UTC).
    pub timestamp: DateTime<Utc>,
    /// List of parent commit hashes.
    /// - Normal commits have 1 parent.
    /// - Merge commits have 2+ parents.
    /// - Initial commits have 0 parents.
    pub parents: Vec<String>,
    /// Optional statistics about changes in this commit.
    ///
    /// This is `None` unless requested via [`GitModel::history_with_diffs`](crate::GitModel::history_with_diffs).
    pub stats: Option<CommitStats>,
    /// List of files changed in this commit.
    ///
    /// This list is **only populated** if the commit was retrieved using [`GitModel::history_with_diffs`](crate::GitModel::history_with_diffs).
    /// Otherwise, it will be an empty vector.
    pub files: Vec<FileChange>,
}

/// Aggregated statistics for a commit.
///
/// # Examples
///
/// ```
/// use git_associates::CommitStats;
///
/// let stats = CommitStats {
///     insertions: 15,
///     deletions: 2,
///     files_changed: 3,
/// };
///
/// assert_eq!(stats.insertions, 15);
/// ```
#[derive(Debug, Clone, Default)]
pub struct CommitStats {
    /// Total number of lines inserted across all files.
    pub insertions: usize,
    /// Total number of lines deleted across all files.
    pub deletions: usize,
    /// Number of files modified, added, or removed.
    pub files_changed: usize,
}

/// Represents changes to a single file within a commit or diff.
///
/// # Examples
///
/// ```
/// use git_associates::FileChange;
///
/// let change = FileChange {
///     path: "src/main.rs".to_string(),
///     extension: "rs".to_string(),
///     insertions: 5,
///     deletions: 1,
///     is_binary: false,
///     hunks: vec![],
/// };
///
/// assert_eq!(change.extension, "rs");
/// ```
#[derive(Debug, Clone)]
pub struct FileChange {
    /// The path of the file (relative to repo root).
    pub path: String,
    /// The file extension (e.g., "rs", "md").
    pub extension: String,
    /// Number of lines added to this file.
    pub insertions: usize,
    /// Number of lines removed from this file.
    pub deletions: usize,
    /// Whether the file is treated as binary by git.
    pub is_binary: bool,
    /// List of hunks (contiguous blocks of changes).
    ///
    /// This list contains the detailed line-by-line diffs.
    /// It is populated when using [`GitModel::history_with_diffs`](crate::GitModel::history_with_diffs) or [`GitModel::diff_workdir`](crate::GitModel::diff_workdir).
    pub hunks: Vec<Hunk>,
}

/// Statistics for a working directory diff (uncommitted changes).
///
/// # Examples
///
/// ```
/// use git_associates::{DiffStats, FileChange};
///
/// let diff_stats = DiffStats {
///     files: vec![],
///     total_added: 42,
///     total_removed: 7,
/// };
///
/// assert_eq!(diff_stats.total_added, 42);
/// ```
#[derive(Debug, Clone, Default)]
pub struct DiffStats {
    /// List of changed files.
    pub files: Vec<FileChange>,
    /// Total lines added across all files.
    pub total_added: usize,
    /// Total lines removed across all files.
    pub total_removed: usize,
}

/// A contiguous block of changes in a file diff.
///
/// A hunk typically starts with a header and contains a mix of context lines,
/// added lines, and removed lines.
///
/// # Examples
///
/// ```
/// use git_associates::{Hunk, LineChange};
///
/// let hunk = Hunk {
///     header: "@@ -1,3 +1,3 @@".to_string(),
///     lines: vec![
///         LineChange::Context("fn main() {".to_string()),
///         LineChange::Removed("    println!(\"Hello\");".to_string()),
///         LineChange::Added("    println!(\"World\");".to_string()),
///         LineChange::Context("}".to_string()),
///     ],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Hunk {
    /// The hunk header line (e.g., `@@ -10,4 +10,5 @@`).
    ///
    /// - `-10,4`: Old file starts at line 10, shows 4 lines.
    /// - `+10,5`: New file starts at line 10, shows 5 lines.
    pub header: String,
    /// The lines within this hunk.
    pub lines: Vec<LineChange>,
}

/// A single line change within a hunk.
///
/// # Examples
///
/// ```
/// use git_associates::LineChange;
///
/// let added = LineChange::Added("let x = 5;".to_string());
/// let removed = LineChange::Removed("let x = 4;".to_string());
/// let context = LineChange::Context("fn foo() {".to_string());
/// ```
#[derive(Debug, Clone)]
pub enum LineChange {
    /// A line that exists in both old and new versions (unchanged).
    ///
    /// Displayed with a leading space ` ` in standard diffs.
    Context(String),
    /// A line that was added in the new version.
    ///
    /// Displayed with a leading `+` in standard diffs.
    Added(String),
    /// A line that was removed from the old version.
    ///
    /// Displayed with a leading `-` in standard diffs.
    Removed(String),
}
