use market_sim::Grid;

// 👺 Havoc: Prove that setting `trade_count` to an extremely large value causes an out-of-memory/capacity panic.
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
            "👺 Havoc: WRECKAGE! Grid::update panics internally on trade_count = usize::MAX due to capacity overflow!"
        );
    }
}

#[test]
#[ignore]
fn havoc_market_sim_alloc_panic_inner() {
    if std::env::args().any(|arg| arg == "havoc_market_sim_alloc_panic_inner") {
        let mut grid = Grid::new(10, 10);
        // By forging the trade_count directly without sanitization, we can cause an allocation overflow!
        grid.trade_count = usize::MAX;

        // This will attempt to call `Vec::with_capacity(usize::MAX)` and instantly crash/panic!
        grid.update();
        std::process::exit(0);
    }
}
