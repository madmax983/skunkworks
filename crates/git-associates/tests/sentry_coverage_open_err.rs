use git_associates::{GitModel, model::LineChange};
use std::fs;
use git2::{Repository, Signature, Oid};
use std::path::Path;

#[test]
fn test_open_err() {
    let temp_dir = std::env::temp_dir().join("git-associates-not-repo");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    let model = GitModel::open(&temp_dir);
    assert!(model.is_err());
}

#[test]
fn test_hunk_origin_cases() {
    let temp_dir = std::env::temp_dir().join("git-associates-hunk-origin");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let repo = Repository::init(&temp_dir).unwrap();

    let file_path = temp_dir.join("test.txt");
    fs::write(&file_path, "a\nb\nc\n").unwrap();

    let mut index = repo.index().unwrap();
    index.add_path(Path::new("test.txt")).unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    let sig = Signature::now("Test", "test@example.com").unwrap();
    let parent_commit = repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[]).unwrap();

    // Context, added, removed
    fs::write(&file_path, "a\nadded\nc\n").unwrap();
    index.add_path(Path::new("test.txt")).unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    let parent = repo.find_commit(parent_commit).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "mod", &tree, &[&parent]).unwrap();

    let model = GitModel::open(&temp_dir).unwrap();
    let history = model.history_with_diffs(1).unwrap();

    let file = &history[0].files[0];
    let hunk = &file.hunks[0];

    let mut has_added = false;
    let mut has_removed = false;
    let mut has_context = false;

    for line in &hunk.lines {
        match line {
            LineChange::Added(_) => has_added = true,
            LineChange::Removed(_) => has_removed = true,
            LineChange::Context(_) => has_context = true,
        }
    }

    assert!(has_added && has_removed && has_context);
}

// Ensure the git_associates::model types format traits run
#[test]
fn test_model_formatting() {
    use git_associates::model::{Commit, CommitStats, FileChange, DiffStats, Hunk, LineChange};
    let c = Commit { hash: "".into(), short_hash: "".into(), author: "".into(), message: "".into(), timestamp: chrono::Utc::now(), parents: vec![], stats: None, files: vec![] };
    let _ = format!("{:?}", c);
    let s = CommitStats::default();
    let _ = format!("{:?}", s);
    let fc = FileChange { path: "".into(), extension: "".into(), insertions: 0, deletions: 0, is_binary: false, hunks: vec![] };
    let _ = format!("{:?}", fc);
    let ds = DiffStats::default();
    let _ = format!("{:?}", ds);
    let h = Hunk { header: "".into(), lines: vec![] };
    let _ = format!("{:?}", h);
    let lc = LineChange::Added("".into());
    let _ = format!("{:?}", lc);
}
