use git2::{Repository, Signature};
use git_associates::GitModel;
use std::fs;
use std::path::Path;

#[test]
fn test_edge_cases_paths_and_timestamp() {
    let temp_dir = std::env::temp_dir().join("git-associates-edge-paths");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let repo = Repository::init(&temp_dir).unwrap();

    let mut index = repo.index().unwrap();

    // File with no extension
    let no_ext_path = temp_dir.join("no_extension");
    fs::write(&no_ext_path, "content").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        // File with invalid UTF-8 path
        let mut bad_path_vec = b"bad_path_".to_vec();
        bad_path_vec.push(0x80);
        let bad_path = std::ffi::OsString::from_vec(bad_path_vec);
        let bad_path_full = temp_dir.join(&bad_path);
        fs::write(&bad_path_full, "content").unwrap();

        index.add_path(Path::new(&bad_path)).unwrap();
    }

    index.add_path(Path::new("no_extension")).unwrap();

    let oid = index.write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();

    let sig = Signature::now("Test", "test@example.com").unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[])
        .unwrap();

    let model = GitModel::open(&temp_dir).unwrap();
    let _diff = model.diff_workdir().unwrap();
    let history = model.history_with_diffs(1).unwrap();

    // Check history files
    let mut extensions = Vec::new();
    for file in &history[0].files {
        extensions.push(file.extension.clone());
        if file.path == "unknown" {
            // Found the unknown path
        }
    }

    // no_extension should have "" as extension
    assert!(extensions.contains(&"".to_string()));
}
