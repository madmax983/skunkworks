use origami::{generate_miura_grid, MiuraParams, Orientation};

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
            "👺 Havoc: WRECKAGE! The Sentry patch is missing or broken!"
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
        // This targets the API directly
        let _ = generate_miura_grid(params, (0, usize::MAX - 1), 0.5);
        std::process::exit(0);
    }
}
//
