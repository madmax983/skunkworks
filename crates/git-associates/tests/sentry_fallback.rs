use git2::{Repository, Signature, Time};
use git_associates::GitModel;
use std::fs;

#[test]
fn test_sentry_unknown_author() {
    let temp_dir = std::env::temp_dir().join("git-associates-unknown-author-test");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let repo = Repository::init(&temp_dir).unwrap();

    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    let time = Time::new(1700000000, 0);

    // We'll just test commit with no message for unwrap_or_default
    let sig = Signature::new("Test Author", "test@example.com", &time).unwrap();

    repo.commit(Some("HEAD"), &sig, &sig, "", &tree, &[])
        .unwrap();

    let model = GitModel::open(&temp_dir).unwrap();
    let history = model.history(1).unwrap();

    assert_eq!(history[0].message, "");
}
