use proptest::prelude::*;
use crate::physics::FluidSolver;

#[test]
fn test_chaos_zero_h() {
    let mut solver = FluidSolver::new(100.0, 100.0);
    solver.h = 0.0;
    solver.add_particle(50.0, 50.0);
    solver.update(0.1);

    // With h=0, NaNs are caught by max(0.0001)
    assert!(!solver.particles[0].rho.is_nan(), "Density became NaN with h=0");
}

#[test]
fn test_chaos_small_h_explosion() {
    let mut solver = FluidSolver::new(100.0, 100.0);
    solver.h = 1e-6; // Small enough to cause absolute underflow in h^9 (1e-54)
    // h = 1e-6
    // h^9 = 1e-54 << f32::MIN_POSITIVE (1e-45 subnormal limit) -> 0.0
    // h^6 = 1e-36 > f32::MIN_POSITIVE (1e-38 is normal min) -> 1e-36 (approx)
    // poly6 = (1/0) * 1e-36 = Infinity

    // We need TWO particles to interact and transmit the Infinite pressure.
    // They must NOT be at the exact same position, or the force loop skips them (r > 0.0 check).
    // AND they must be resolvable by f32.
    // At x=50.0, machine epsilon is ~4e-6. We can't place them within h=1e-6!
    // So we move to 0.0 where precision is finer.
    solver.add_particle(0.0, 0.0);
    solver.add_particle(1e-10, 0.0); // 1e-10 is distinct from 0.0

    solver.update(0.1);

    let p = &solver.particles[0];
    assert!(!p.x.is_infinite(), "x became Infinite with h=1e-6");
    assert!(!p.x.is_nan(), "x became NaN with h=1e-6");
}

proptest! {
    #[test]
    fn test_chaos_simulation(
        dt in -2.0f32..2.0,
        h in 1e-7f32..1e-5f32, // Target the weak spot (underflow range)
        k in 0.0f32..100.0,
        mu in 0.0f32..10.0,
        particles in prop::collection::vec((-50.0f32..150.0, -50.0f32..150.0), 0..20)
    ) {
        let mut solver = FluidSolver::new(100.0, 100.0);
        solver.h = h;
        solver.k = k;
        solver.mu = mu;

        for (x, y) in particles {
            solver.add_particle(x, y);
        }

        if !dt.is_finite() {
            return Ok(());
        }

        solver.update(dt);

        for p in &solver.particles {
             prop_assert!(!p.x.is_nan(), "x became NaN with dt={}, h={}, k={}, mu={}", dt, h, k, mu);
             prop_assert!(!p.y.is_nan(), "y became NaN");
             prop_assert!(!p.vx.is_nan(), "vx became NaN");
             prop_assert!(!p.vy.is_nan(), "vy became NaN");

             prop_assert!(!p.x.is_infinite(), "x became Infinite with dt={}, h={}, k={}", dt, h, k);
             prop_assert!(!p.y.is_infinite(), "y became Infinite");
        }
    }
}
