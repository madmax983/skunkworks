#[cfg(test)]
mod tests {
    use crate::vm::nova_attractor::AttractorState;

    #[test]
    fn test_attractor_step_lorenz() {
        let mut attractor = AttractorState::new();
        // Default is Lorenz
        let initial_x = attractor.x;
        attractor.step();
        assert_ne!(attractor.x, initial_x);
        assert_eq!(attractor.history.len(), 1);
    }

    #[test]
    fn test_attractor_modes() {
        let mut attractor = AttractorState::new();
        attractor.mode = 1; // Rossler
                            // Initialize appropriate params for Rossler manually if not calling init op
                            // But AttractorState::new() sets Lorenz params.
                            // We should set params to verify step logic isn't NAN or same.
        attractor.sigma = 0.2;
        attractor.rho = 0.2;
        attractor.beta = 5.7;

        attractor.step();
        let r_x = attractor.x;

        // Thomas
        attractor.mode = 2;
        attractor.x = 0.1;
        attractor.y = 0.0;
        attractor.z = 0.0;
        attractor.sigma = 0.19; // b
        attractor.step();

        assert_ne!(attractor.x, r_x);
    }
}
