use market_sim::Grid;

#[test]
fn test_havoc_market_sim_overflow() {
    // 👺 HAVOC: Proving that Grid::new panics if given invalid indices due to size overflow.
    // The method calculates `width * height` without checking if the result exceeds
    // the system's capacity, which causes a deterministic DoS panic.
    let grid = Grid::new(usize::MAX, 2);
    assert_eq!(grid.width, 0);
    assert_eq!(grid.height, 0);
}
