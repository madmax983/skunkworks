use git2::{Repository, Signature, Time};
use git_associates::GitModel;
use std::fs;

#[test]
fn test_history_without_diffs_limit() {
    let temp_dir = std::env::temp_dir().join("git-associates-history-limit");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let repo = Repository::init(&temp_dir).unwrap();

    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    let time = Time::new(1700000000, 0);
    let sig = Signature::new("Test", "test@example.com", &time).unwrap();

    repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
        .unwrap();

    let model = GitModel::open(&temp_dir).unwrap();
    let history = model.history(10).unwrap();
    assert_eq!(history.len(), 1);

    // Limit coverage check inside diff_workdir branch
    let changes = model.diff_workdir().unwrap();
    assert_eq!(changes.files.len(), 0);
    assert_eq!(changes.total_added, 0);
    assert_eq!(changes.total_removed, 0);
}
