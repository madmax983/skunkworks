use chimera_lang::prelude::*;
use flocking::FlockingParams;
use macroquad::prelude::*;
use physics_pbd::PbdSystem;

pub struct MagneticScavenger {
    pub vm: ChimeraVM,
    pub particle_indices: Vec<usize>, // 0 is Center, 1..N are outer rim
    pub actuators: Vec<usize>,        // Constraints that are muscles
    pub magnetism: f32,               // 0.0 (North) to 1.0 (South), 0.5 Neutral
    pub color: Color,
    pub flocking_params: FlockingParams,
    pub energy: f32,
}

impl MagneticScavenger {
    pub fn new(
        system: &mut PbdSystem,
        pos: Vec2,
        dna: Dna,
    ) -> Self {
        let center_pos = vec3(pos.x, pos.y, 0.0);
        let center_idx = system.add_particle(center_pos, 1.0); // Mass 1.0
        let mut particle_indices = vec![center_idx];
        let mut actuators = Vec::new();

        // Create a small shape (Triangle or Diamond)
        let radius = 2.0;
        let num_tentacles = 3;
        for i in 0..num_tentacles {
            let angle = (i as f32 / num_tentacles as f32) * std::f32::consts::TAU;
            let offset = vec3(angle.cos() * radius, angle.sin() * radius, 0.0);
            let p_idx = system.add_particle(center_pos + offset, 0.5); // Lighter tentacles
            particle_indices.push(p_idx);

            // Connect to center (Muscle)
            let c_idx = system.constraints.len();
            system.add_actuator_constraint(center_idx, p_idx, radius * 0.5, radius * 1.5, 0.5);
            actuators.push(c_idx);

            // Connect to neighbor tentacle (Structural)
            if i > 0 {
                let prev = particle_indices[i];
                system.add_distance_constraint(
                    prev,
                    p_idx,
                    radius * 2.0 * (std::f32::consts::PI / num_tentacles as f32).sin(),
                );
            }
        }
        // Close loop
        let first = particle_indices[1];
        let last = particle_indices[num_tentacles];
        system.add_distance_constraint(
            first,
            last,
            radius * 2.0 * (std::f32::consts::PI / num_tentacles as f32).sin(),
        );

        let r = ::rand::random::<f32>() * 0.5 + 0.5;
        let g = ::rand::random::<f32>() * 0.2;
        let b = ::rand::random::<f32>() * 0.5;

        Self {
            vm: ChimeraVM::new(dna),
            particle_indices,
            actuators,
            magnetism: 0.5,
            color: Color::new(r, g, b, 1.0), // Red-ish for scavengers
            flocking_params: FlockingParams {
                view_radius: 30.0,
                separation_radius: 8.0,
                max_speed: 20.0,
                max_force: 1.0,
                separation_weight: 2.0,
                alignment_weight: 1.0,
                cohesion_weight: 1.0,
            },
            energy: 100.0,
        }
    }

    pub fn center_pos(&self, system: &PbdSystem) -> Vec3 {
        system.particles[self.particle_indices[0]].pos
    }

    pub fn center_vel(&self, system: &PbdSystem) -> Vec3 {
        system.particles[self.particle_indices[0]].vel
    }
}

pub fn create_scavenger_dna() -> Dna {
    // Logic: Piezo-Magnetic Pulsing
    // Inputs: [Strain, SelfMag]
    // Outputs: [Contraction, NewMag]

    let genes = vec![
        // Stack: [Strain, SelfMag]

        // --- Calculate NewMag ---
        Gene {
            op: OpCode::Dup,
            args: vec![],
        }, // [Strain, SelfMag, SelfMag]
        // Mag Drift: NewMag = (SelfMag + 1) % 100
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Mod,
            args: vec![],
        }, // [Strain, SelfMag, NewMag]
        // Arrange Stack for Contraction
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [Strain, NewMag, SelfMag]
        Gene {
            op: OpCode::Drop,
            args: vec![],
        }, // [Strain, NewMag]
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [NewMag, Strain]
        // --- Calculate Contraction ---
        // Stack: [NewMag, Strain]
        // If Strain > 50 (Stretched), Contract (20). Else Relax (100).
        Gene {
            op: OpCode::Dup,
            args: vec![],
        }, // [NewMag, Strain, Strain]
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Gt,
            args: vec![],
        }, // [NewMag, Strain, IsStretched]
        // Map Bool(0/1) to Factor(100/20)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(80)],
        },
        Gene {
            op: OpCode::Mul,
            args: vec![],
        }, // [NewMag, Strain, Offset]
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [NewMag, Strain, 100, Offset]
        Gene {
            op: OpCode::Sub,
            args: vec![],
        }, // [NewMag, Strain, Contraction]
        // Cleanup Strain
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [NewMag, Contraction, Strain]
        Gene {
            op: OpCode::Drop,
            args: vec![],
        }, // [NewMag, Contraction]
        // Final Return Order
        Gene {
            op: OpCode::Swap,
            args: vec![],
        }, // [Contraction, NewMag]
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
        evolution_config: None,
    }
}
