//! 🧬 Splice: Cross `miller-lattice` × `platter`
//!
//! Concept: Codebase Thermodynamic Shadow.
//!
//! The discrete 3D crystalline lattice of a filesystem structure (`miller-lattice`)
//! is projected down onto a continuous 2D scalar heat field (`platter`).
//! The structure acts as a thermodynamic heatsink or emitter. High-density directory
//! clusters cast a hot shadow, while sparse files barely register.
//!
//! This provides an emergent 2D thermodynamic visualization of 3D hierarchical codebase
//! complexity.

use miller_lattice::Crystal;
use platter::Platter;

pub struct ThermodynamicShadow {
    pub crystal: Crystal,
    pub platter: Platter,
}

impl ThermodynamicShadow {
    pub fn new(width: usize, height: usize, crystal: Crystal) -> Self {
        Self {
            crystal,
            platter: Platter::new(width, height),
        }
    }

    pub fn project(&mut self) {
        let width = self.platter.width() as i32;
        let height = self.platter.height() as i32;

        let cx = width / 2;
        let cy = height / 2;

        for atom in &self.crystal.atoms {
            let px = cx + atom.position.x;
            let py = cy + atom.position.y;

            // If the 3D lattice point projects into the 2D platter bounds
            if px >= 0 && px < width && py >= 0 && py < height {
                let heat_value = if atom.is_dir {
                    2.0 // Directories act as strong heat emitters
                } else {
                    0.5 // Files act as weak emitters
                };

                self.platter
                    .accumulate(px as usize, py as usize, heat_value);
            }
        }
    }

    pub fn step_decay(&mut self, rate: f64) {
        self.platter.decay(rate);
    }
}
