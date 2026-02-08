use ink_jet::fluid::FluidSolver;

#[test]
fn test_density_addition() {
    let size = 10;
    let mut fluid = FluidSolver::new(size, 0.1, 0.0, 0.0);

    fluid.add_density(5, 5, 100.0);

    let idx = 5 + 5 * size;
    assert!(fluid.density[idx] > 0.0, "Density was not added");
}

#[test]
fn test_conservation() {
    let size = 10;
    // No diffusion, so mass should be strictly conserved
    let mut fluid = FluidSolver::new(size, 0.1, 0.0, 0.0);

    fluid.add_density(5, 5, 100.0);

    let initial_mass: f32 = fluid.density.iter().sum();

    fluid.step();

    let final_mass: f32 = fluid.density.iter().sum();

    if initial_mass == 0.0 {
        assert!(false, "Initial mass is 0, add_density failed");
    }

    assert!((initial_mass - final_mass).abs() < 0.1, "Mass not conserved");
}
