use miller_lattice::Crystal;
use std::path::Path;

#[test]
fn havoc_miller_paths() {
    let binding = "/".repeat(100_000);
    let p = Path::new(binding.as_str());
    let _ = Crystal::build_from_path(p);
}
