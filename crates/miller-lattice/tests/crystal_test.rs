use miller_lattice::Crystal;
use std::path::Path;

#[test]
fn test_crystal_build_from_src_dir() {
    let src_path = Path::new("./src");

    // Only run the test if the directory actually exists (e.g. running from crate root vs workspace root)
    if src_path.exists() && src_path.is_dir() {
        let crystal = Crystal::build_from_path(src_path).expect("Should build crystal from src directory");
        assert!(!crystal.atoms.is_empty(), "Crystal built from src should have atoms");
    }
}
