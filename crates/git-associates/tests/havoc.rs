use git_associates::GitModel;

#[test]
fn havoc_test_alloc() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_alloc_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: WRECKAGE! The Sentry patch is missing or broken!"
        );
    }
}

#[test]
#[ignore]
fn havoc_test_alloc_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_alloc_inner") {
        let temp_dir = std::env::temp_dir().join("git-associates-havoc-crash2");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();
        let repo = git2::Repository::init(&temp_dir).unwrap();

        let mut index = repo.index().unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();

        repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
            .unwrap();

        let model = GitModel::open(&temp_dir).unwrap();

        // This targets the API directly
        let _ = model.history(usize::MAX);

        std::process::exit(0);
    }
}
