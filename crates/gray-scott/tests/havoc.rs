use gray_scott::GrayScott;

#[test]
fn havoc_gray_scott_init_overflow() {
    // 👺 Havoc: Using size large enough to overflow usize during initialization
    let _ = GrayScott::new(usize::MAX, 2);
}

#[test]
#[should_panic(expected = "coordinate out of bounds")]
fn havoc_gray_scott_scanline_wrapping() {
    let gs = GrayScott::new(10, 10);
    let _ = gs.get_index(15, 0);
}

// 👺 Havoc: `chunks_exact_mut(0)` panics!
#[test]
fn havoc_gray_scott_zero_width_panic() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_gray_scott_zero_width_panic_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: WRECKAGE! update_sequential panics internally on width = 0 due to chunks_exact_mut(0)!"
        );
    }
}

#[test]
#[ignore]
fn havoc_gray_scott_zero_width_panic_inner() {
    if std::env::args().any(|arg| arg == "havoc_gray_scott_zero_width_panic_inner") {
        let mut gs = GrayScott::new(0, 10);
        // This will call `chunks_exact_mut(0)` which panics!
        gs.update(0.1, 0.1, 1.0);
        std::process::exit(0);
    }
}
