#[cfg(test)]
mod tests {
    use crate::model::{Grid, Termite, Blueprint};

    #[test]
    fn test_heat_diffusion() {
        let mut grid = Grid::new(10, 10);
        // Set heat at center
        grid.heat[55] = 100.0;

        // Update (stub does nothing)
        grid.update_heat();

        // Check neighbor (should have heat > 0 if diffusion works)
        assert!(grid.heat[54] > 0.0, "Heat should diffuse to left neighbor");
    }

    #[test]
    fn test_termite_build() {
        let mut grid = Grid::new(10, 10);
        let mut termite = Termite::new(5.5, 5.5);
        termite.blueprint.build_threshold = 0.5;
        termite.blueprint.randomness = 0.0; // Deterministic behavior

        // Set high heat at termite location (5, 5)
        grid.heat[55] = 1.0;

        // Update termite (stub does nothing)
        termite.update(&mut grid);

        // Expect wall to be built at (5, 5)
        assert!(grid.wall[55], "Termite should build wall when heat > threshold");
    }

    #[test]
    fn test_consensus() {
        let mut t1 = Termite::new(0.0, 0.0);
        t1.fitness = 100.0;
        t1.blueprint.build_threshold = 0.9; // Unique value

        let mut t2 = Termite::new(0.0, 0.0);
        t2.fitness = 0.0;
        t2.blueprint.build_threshold = 0.1; // Different value

        // Interact (stub does nothing)
        t2.interact(&t1);

        // Expect t2 to adopt t1's blueprint
        assert_eq!(t2.blueprint.build_threshold, 0.9, "Termite should adopt better blueprint");
    }
}
