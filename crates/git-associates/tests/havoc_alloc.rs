use git2::{Repository, Signature};
use git_associates::GitModel;

#[test]

fn havoc_test_alloc() {
    let temp_dir = std::env::temp_dir().join("git-associates-havoc-alloc");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();
    let repo = Repository::init(&temp_dir).unwrap();

    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    let sig = Signature::now("Test", "test@example.com").unwrap();

    repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
        .unwrap();

    // Can we crash GitModel::history with a huge history limit request?
    // Yes! GitModel::history uses Vec::with_capacity(limit).
    // An attacker passing usize::MAX causes a panic.
    let model = GitModel::open(&temp_dir).unwrap();

    // Pass usize::MAX to history(limit).
    // `with_capacity(usize::MAX)` will panic with an capacity overflow error.
    let _ = model.history(usize::MAX);
}
