use market_sim::Grid;

#[test]
fn havoc_market_sim_alloc_panic() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_market_sim_alloc_panic_inner")
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
fn havoc_market_sim_alloc_panic_inner() {
    if std::env::args().any(|arg| arg == "havoc_market_sim_alloc_panic_inner") {
        let mut grid = Grid::new(10, 10);
        grid.trade_count = usize::MAX;

        // This targets the API directly
        let _ = grid.update();

        std::process::exit(0);
    }
}
//
