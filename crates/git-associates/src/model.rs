//! # Git Data Models
//!
//! Core data structures representing git repository entities.
//!
//! This module provides the [`Commit`], [`CommitStats`], and [`FileChange`] types, which are
//! used to construct an application-agnostic view of a git repository's history and diffs.
//! These structs are designed to hold the data parsed by the `GitModel` interface.

use chrono::{DateTime, Utc};

/// Represents a single commit in the git history.
///
/// This struct aggregates essential metadata about a commit, including its hash,
/// author, message, timestamp, and optional diff statistics.
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
/// use git_associates::model::{Hunk, LineChange};
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
/// use git_associates::model::LineChange;
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
