pub mod lorenz;
pub mod monitor;

#[cfg(test)]
mod tests {
    use super::lorenz::*;
    use super::monitor::*;

    #[test]
    fn test_lorenz_step_movement() {
        let system = LorenzSystem::new(10.0, 28.0, 8.0/3.0);
        let initial_state = LorenzState { x: 1.0, y: 1.0, z: 1.0 };
        let dt = 0.01;

        let _next_state = system.step(initial_state, dt);

        let state = LorenzState { x: 1.0, y: 2.0, z: 3.0 };
        let next = system.step(state, dt);

        assert!((next.x - 1.1).abs() < 0.1, "X should be approx 1.1, got {}", next.x);
        assert!((next.y - 2.23).abs() < 0.1, "Y should be approx 2.23, got {}", next.y);
        assert!((next.z - 2.94).abs() < 0.1, "Z should be approx 2.94, got {}", next.z);

        assert!(next.x != state.x || next.y != state.y || next.z != state.z);
    }

    #[test]
    fn test_monitor_ranges() {
        let mut monitor = SystemMonitor::new();
        monitor.update(); // First update might be 0 for CPU
        // We can't guarantee CPU usage > 0, but we can guarantee ranges.

        let sigma = monitor.get_sigma();
        assert!(sigma >= 10.0, "Sigma should be >= 10.0");

        let rho = monitor.get_rho();
        assert!(rho >= 28.0, "Rho should be >= 28.0");

        let beta = monitor.get_beta();
        assert!(beta >= 2.6, "Beta should be >= 2.6");
    }
}
