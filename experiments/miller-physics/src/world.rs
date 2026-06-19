use glam::f32::Vec3;
use miller_lattice::Crystal;
use physics_pbd::PbdSystem;
use rustc_hash::FxHashMap;

pub struct World {
    pub system: PbdSystem,
    pub width: f64,
    pub height: f64,
    pub bonds: Vec<(usize, usize)>,
    pub is_dir: Vec<bool>,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut system = PbdSystem::new();
        let mut bonds = Vec::new();
        let mut is_dir = Vec::new();

        // Load the codebase crystal structure
        let path = std::env::current_dir().unwrap_or_else(|_| ".".into());
        let crystal = Crystal::build_from_path(&path).unwrap_or_else(|_| Crystal {
            atoms: vec![],
            bonds: vec![],
            lookup: FxHashMap::default(),
        });

        // Translate atoms to physics particles
        for atom in &crystal.atoms {
            // Directories are heavier, files are lighter
            let mass = if atom.is_dir { 5.0 } else { 1.0 };

            // Project the 3D lattice coordinates into the 2D physics space, scaling them to spread them out
            let scale = 15.0;
            let pos = Vec3::new(
                (atom.position.x as f32) * scale,
                (atom.position.y as f32) * scale,
                (atom.position.z as f32) * scale,
            );

            let id = system.add_particle(pos, mass);
            is_dir.push(atom.is_dir);

            // Add a pin constraint to the root to anchor the entire structure
            if atom.position.x == 0 && atom.position.y == 0 && atom.position.z == 0 {
                 let _ = system.add_pin_constraint(id, pos);
            }
        }

        // Translate lattice bonds to distance constraints
        for bond in &crystal.bonds {
            bonds.push((bond.0, bond.1));
            let _ = system.add_distance_constraint(bond.0, bond.1, 15.0); // Rest length of 15.0
        }

        Self {
            system,
            width,
            height,
            bonds,
            is_dir,
        }
    }

    pub fn update(&mut self) {
        // Step the PBD physics simulation
        self.system.step(0.1, 5);

        // Apply a gentle centering gravity to keep the structure somewhat contained
        for particle in &mut self.system.particles {
            if particle.inv_mass > 0.0 {
                let gravity_force = -particle.pos * 0.01;
                particle.vel += gravity_force;
            }
        }
    }
}
