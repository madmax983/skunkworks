use origami::{generate_miura_grid, MiuraParams, Orientation};

// 👺 Havoc: `generate_miura_grid` can panic with `usize::MAX - 1` due to checked math passing, but capacity still overflowing!
#[test]
fn havoc_origami_capacity_panic() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_origami_capacity_panic_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: WRECKAGE! generate_miura_grid panics internally on size due to capacity overflow!"
        );
    }
}

#[test]
#[ignore]
fn havoc_origami_capacity_panic_inner() {
    if std::env::args().any(|arg| arg == "havoc_origami_capacity_panic_inner") {
        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 1.4,
            orientation: Orientation::Horizontal,
        };
        // By choosing rows=usize::MAX-1 and cols=0:
        // rows.checked_add(1) = usize::MAX
        // cols.checked_add(1) = 1
        // r.checked_mul(c) = usize::MAX * 1 = usize::MAX
        // Then `Vec::with_capacity(usize::MAX)` panics!
        let _ = generate_miura_grid(params, (0, usize::MAX - 1), 0.5);
        std::process::exit(0);
    }
}
