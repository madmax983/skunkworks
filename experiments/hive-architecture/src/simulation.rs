use crate::model::{Grid, Termite};
use fastrand;
use macroquad::prelude::Color;

pub struct World {
    pub grid: Grid,
    pub termites: Vec<Termite>,
    pub width: usize,
    pub height: usize,
    pub step: u64,
}

impl World {
    pub fn new(width: usize, height: usize, num_termites: usize) -> Self {
        let grid = Grid::new(width, height);

        let mut termites = Vec::with_capacity(num_termites);
        for _ in 0..num_termites {
            let x = fastrand::f32() * width as f32;
            let y = fastrand::f32() * height as f32;
            let mut t = Termite::new(x, y);
            // Randomize genetics
            t.blueprint.build_threshold = 0.3 + fastrand::f32() * 0.4;
            t.blueprint.destroy_threshold = (t.blueprint.build_threshold - 0.1).max(0.0);
            t.blueprint.color = Color::new(fastrand::f32(), fastrand::f32(), fastrand::f32(), 1.0);
            termites.push(t);
        }
        Self {
            grid,
            termites,
            width,
            height,
            step: 0,
        }
    }

    pub fn update(&mut self) {
        self.step += 1;

        // Add Heat to Server (Center Block)
        let cx = self.width / 2;
        let cy = self.height / 2;
        let r = 20;
        // Check bounds
        if cx > r && cy > r && cx + r < self.width && cy + r < self.height {
             for y in cy-r..cy+r {
                for x in cx-r..cx+r {
                    self.grid.set_heat(x, y, 1.0);
                }
            }
        }

        // Diffusion (Parallel)
        self.grid.update_heat();

        // Termites Update (Sequential for Grid Safety)
        for t in &mut self.termites {
            t.update(&mut self.grid);
        }

        // Interaction (Stochastic Subset)
        // Parallel interaction is hard because of borrow checker (pairing mutable references).
        // We can do it in chunks or just pick random pairs.
        // Let's iterate a few times and pick random pairs.
        let num_interactions = self.termites.len() / 10;
        for _ in 0..num_interactions {
            let i = fastrand::usize(..self.termites.len());
            let j = fastrand::usize(..self.termites.len());
            if i != j {
                // To borrow two elements mutably from a Vec is tricky.
                // Use unsafe or split_at_mut.
                // Or just swap/clone.
                // Since interaction only reads fitness and writes blueprint (Copy),
                // we can read one and write to other.

                // Let's do a simple swap trick or unsafe for performance,
                // but let's stick to safe Rust:
                if i < j {
                    let (left, right) = self.termites.split_at_mut(j);
                    // left[i] interacts with right[0] (which was originally j)
                    left[i].interact(&right[0]);
                    right[0].interact(&left[i]);
                } else {
                    let (left, right) = self.termites.split_at_mut(i);
                    left[j].interact(&right[0]);
                    right[0].interact(&left[j]);
                }
            }
        }
    }
}
