use crate::physics::FluidSolver;
use rand::Rng;

pub struct App {
    pub solver: FluidSolver,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let mut solver = FluidSolver::new(100.0, 100.0);
        // Add some initial particles in a block
        for i in 0..10 {
            for j in 0..20 {
                solver.add_particle(30.0 + i as f32 * 1.5, 10.0 + j as f32 * 1.5);
            }
        }

        Self {
            solver,
            should_quit: false,
        }
    }

    pub fn tick(&mut self) {
        // Physics step
        self.solver.update(0.1);
    }

    pub fn spawn_particles(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            self.solver.add_particle(
                50.0 + rng.gen_range(-5.0..5.0),
                10.0 + rng.gen_range(-5.0..5.0),
            );
        }
    }

    pub fn reset(&mut self) {
        self.solver.particles.clear();
        for i in 0..10 {
            for j in 0..20 {
                self.solver
                    .add_particle(30.0 + i as f32 * 1.5, 10.0 + j as f32 * 1.5);
            }
        }
    }
}
