use proptest::prelude::*;
use std::fs;
use tempfile::tempdir;

// Testing the public api since internal functions are private
use miller_lattice::Crystal;

proptest! {
    #[test]
    fn test_build_from_path_havoc(
        dir_name in "[a-zA-Z0-9_.-]+",
        file_name in "[a-zA-Z0-9_.-]+"
    ) {
        // Use tempfile to ensure directory is deleted on panic or drop
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path();

        let sub_dir = path.join(&dir_name);
        let _ = fs::create_dir_all(&sub_dir);

        let file_path = sub_dir.join(&file_name);
        let _ = fs::write(&file_path, "chaotic data");

        let _crystal = Crystal::build_from_path(path);
    }
}
