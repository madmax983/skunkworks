use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub parents: Vec<String>,
    pub stats: Option<CommitStats>,
    pub files: Vec<FileChange>,
}

#[derive(Debug, Clone, Default)]
pub struct CommitStats {
    pub insertions: usize,
    pub deletions: usize,
    pub files_changed: usize,
}

#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: String,
    pub extension: String,
    pub insertions: usize,
    pub deletions: usize,
    pub is_binary: bool,
    pub hunks: Vec<Hunk>,
}

#[derive(Debug, Clone, Default)]
pub struct DiffStats {
    pub files: Vec<FileChange>,
    pub total_added: usize,
    pub total_removed: usize,
}

#[derive(Debug, Clone)]
pub struct Hunk {
    pub header: String,
    pub lines: Vec<LineChange>,
}

#[derive(Debug, Clone)]
pub enum LineChange {
    Context(String),
    Added(String),
    Removed(String),
}
