use biomorphic_clock::simulation::Grid;

#[test]
fn test_anisotropic_diffusion() {
    let width = 11;
    let height = 11;
    let mut grid = Grid::new(width, height);

    // Seed center
    let center_x = width / 2;
    let center_y = height / 2;
    grid.seed(center_x, center_y);

    // Initial check: only center has v=1.0
    let center_idx = center_y * width + center_x;
    assert_eq!(grid.v[center_idx], 1.0, "Center should be seeded");

    // Run update with angle 0.0 (horizontal diffusion preferred)
    // We expect diffusion along X to be stronger than along Y.
    // So v(center_x+1, center_y) > v(center_x, center_y+1)

    // Run enough steps for diffusion to occur but not reach boundaries
    for _ in 0..10 {
        grid.update(1.0, 0.0);
    }

    let idx_right = center_y * width + (center_x + 1);
    let idx_down = (center_y + 1) * width + center_x;

    let v_right = grid.v[idx_right];
    let v_down = grid.v[idx_down];

    println!("v_right: {}, v_down: {}", v_right, v_down);

    // Assert anisotropic behavior
    // With angle 0, diffusion should be primarily horizontal.
    assert!(v_right > v_down, "Expected horizontal diffusion > vertical diffusion for angle 0.0. Got right: {}, down: {}", v_right, v_down);
}
