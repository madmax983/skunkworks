use market_sim::Grid;

#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn test_havoc_market_sim_overflow() {
    // 👺 HAVOC: Proving that Grid::new panics if given invalid indices due to size overflow.
    // The method calculates `width * height` without checking if the result exceeds
    // the system's capacity, which causes a deterministic DoS panic.
    let _grid = Grid::new(usize::MAX, 2);
}
