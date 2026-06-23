use arthropod::Button;

#[test]
fn havoc_test_arthropod_panic() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_arthropod_panic_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status()
        .expect("Failed to execute subprocess");

    assert!(
        !status.success(),
        "👺 Havoc: WRECKAGE! The button didn't panic! Our chaos failed!"
    );
}

#[test]
#[ignore]
fn havoc_test_arthropod_panic_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_arthropod_panic_inner") {
        let button = Button::new("a", f32::MAX, f32::MAX, f32::MAX, f32::MAX);
        let _ = button.draw();
        std::process::exit(0);
    }
}
