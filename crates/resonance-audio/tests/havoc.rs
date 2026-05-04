use resonance_audio::physics::PhysicsGrid;

#[test]
fn test_zero_dim() {
    let mut grid = PhysicsGrid::new(0, 0);
    grid.step();
}

#[test]
fn test_one_dim() {
    let mut grid = PhysicsGrid::new(1, 1);
    grid.step();
}

#[test]
fn havoc_resonance_overflow() {
    // 👺 Havoc: `PhysicsGrid::new` calculates `size = width * height` without checked_mul.
    // An attacker or erroneous logic providing large dimensions will cause a deterministic panic DoS.
    let _grid = PhysicsGrid::new(usize::MAX, 2);
}

// 👺 Havoc: `chunks_exact_mut(0)` panics!
#[test]
fn havoc_resonance_zero_width_panic() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_resonance_zero_width_panic_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: WRECKAGE! PhysicsGrid::step panics internally on width = 0 due to chunks_exact_mut(0)!"
        );
    }
}

#[test]
#[ignore]
fn havoc_resonance_zero_width_panic_inner() {
    if std::env::args().any(|arg| arg == "havoc_resonance_zero_width_panic_inner") {
        let mut grid = PhysicsGrid::new(0, 10);
        grid.step();
        std::process::exit(0);
    }
}
