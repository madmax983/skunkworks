use market_sim::Grid;

#[test]
#[should_panic(expected = "capacity overflow")]
fn test_capacity_overflow_dos() {
    // 🧨 The Trigger: Width = usize::MAX, Height = 0
    // The capacity checks `width.checked_mul(height)` which equals 0, preventing immediate OOM.
    // However, `scan_x: (0..width).collect()` loops `usize::MAX` times, which triggers a capacity overflow.
    let _grid = Grid::new(usize::MAX, 0);
}
