use market_sim::{Grid, Particle};
use std::env;
use std::process::Command;

#[test]
fn havoc_market_sim() {
    let mut grid = Grid::new(100, 100);
    // basic test
    grid.set(50, 50, Particle::Bid(1));
    grid.update();
}

#[test]
fn havoc_market_sim_public_fields_oob_inner() {
    if env::var("HAVOC_INNER").is_ok() {
        let mut grid = Grid::new(10, 10);
        // Havoc: mutate public width to break invariant between width/height and internal Vecs
        grid.width = 100;
        // Detonate! This will cause an out-of-bounds access panic because the internal
        // vectors (cells, updated) were not resized to match the new width.
        grid.update();
        return;
    }

    let output = Command::new(env::current_exe().unwrap())
        .args([
            "--test",
            "havoc_market_sim_public_fields_oob_inner",
            "--exact",
        ])
        .env("HAVOC_INNER", "1")
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "👺 Havoc SUCCESS: The inner process should have panicked due to OOB access."
    );
}

#[test]
fn havoc_market_sim_public_fields_cells_inner() {
    if env::var("HAVOC_INNER").is_ok() {
        let mut grid = Grid::new(10, 10);
        // Havoc: clear the public cells array, but leave width/height intact.
        grid.cells.clear();

        // Detonate! This will cause an out-of-bounds panic when indexing cells.
        grid.update();
        return;
    }

    let output = Command::new(env::current_exe().unwrap())
        .args([
            "--test",
            "havoc_market_sim_public_fields_cells_inner",
            "--exact",
        ])
        .env("HAVOC_INNER", "1")
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "👺 Havoc SUCCESS: The inner process should have panicked due to OOB access on cleared cells."
    );
}
