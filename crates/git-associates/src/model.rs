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
    /// This is empty unless requested via [`GitModel::history_with_diffs`](crate::GitModel::history_with_diffs).
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
    /// This may be empty depending on how the diff was generated.
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
/// A hunk typically starts with a header (e.g., `@@ -1,5 +1,5 @@`) and contains
/// a mix of context lines, added lines, and removed lines.
#[derive(Debug, Clone)]
pub struct Hunk {
    /// The hunk header line (e.g., `@@ -10,4 +10,5 @@`).
    pub header: String,
    /// The lines within this hunk.
    pub lines: Vec<LineChange>,
}

/// A single line change within a hunk.
#[derive(Debug, Clone)]
pub enum LineChange {
    /// A line that exists in both old and new versions (context).
    Context(String),
    /// A line that was added in the new version.
    Added(String),
    /// A line that was removed from the old version.
    Removed(String),
}
