// No lib exposed, let's include the mod
#[path = "../src/level_gen.rs"]
mod level_gen;

#[test]
fn havoc_test_unwrap() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_unwrap_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: WRECKAGE! generate_level panics internally due to unwrap()!"
        );
    }
}

#[test]
#[ignore]
fn havoc_test_unwrap_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_unwrap_inner") {
        let p = std::path::Path::new("/");
        let _ = level_gen::generate_level(p); // Trigger unwrap on root parent()
        std::process::exit(0);
    }
}
