#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    #[cfg(feature = "nova")]
    fn test_optics_reflector_reflection() {
        // [ Reflector(1, 5, 5) Fire(1, 0, 1) ]
        // Reflector at 5,5 (Orientation 1: Horizontal -)
        // Fire at 5,5 (from 0,0 presumably, but let's see where context is)
        // Wait, Fire uses context_loc as origin.
        // Mirror places at target y,x.

        // Plan:
        // 1. Place Mirror at (0, 5). Orientation 0 (| Vertical).
        // 2. Fire projectile from (0, 0) with vector (0, 1) [Right].
        //    Wait, (0, 1) is East. (1, 0) is South.
        //    Projectile moves.
        //    If it hits Mirror at (0, 5), it should reflect.
        //    Vertical mirror reflects X velocity. vx = -vx.
        //    So (0, 1) [vx=1] becomes (0, -1) [vx=-1].

        let genes = vec![
            // Place Mirror at (0, 5)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Orientation |
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // y
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // x
            },
            Gene {
                op: OpCode::Reflector,
                args: vec![],
            },
            // Fire from (0, 0) towards (0, 5)
            // Fire args: dx, dy, power
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Power
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // dy (0)
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // dx (1) -> East
            },
            Gene {
                op: OpCode::Fire,
                args: vec![],
            },
        ];

        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;
        vm.context_loc = (0, 0);

        // Execute placement and fire
        for _ in 0..8 {
            vm.step();
        }

        // Verify Reflector exists
        match &vm.grid[0][5] {
            Value::Str(s) => assert_eq!(s, "REFLECTOR:0"),
            _ => panic!("Reflector not placed"),
        }

        // Verify Projectile exists
        assert_eq!(vm.projectiles.len(), 1);
        let p = &vm.projectiles[0];
        assert!(p.vx > 0.9); // Moving East

        // Run simulation to let projectile hit mirror
        // Distance is 5. Speed is 1. Should hit in ~5 ticks.
        for _ in 0..6 {
            crate::vm::nova_ballistics::update_projectiles(&mut vm);
        }

        // Check if reflected
        // Projectile should still be alive (surviving logic)
        assert_eq!(vm.projectiles.len(), 1);
        let p_after = &vm.projectiles[0];

        // Should be moving West now (vx < 0)
        assert!(
            p_after.vx < -0.9,
            "Projectile did not reflect: vx={}",
            p_after.vx
        );
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_optics_prism_split() {
        // Place Prism at (5, 5). Fire at it from (5, 0).
        // Should result in 3 projectiles.

        let genes = vec![
            // Place Prism
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // y=5
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // x=5
            },
            Gene {
                op: OpCode::Prism,
                args: vec![],
            },
            // Fire from (5, 0) - we want x=5, y=0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // dy=0
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // dx=5
            },
            Gene {
                op: OpCode::Migrate, // Move to (0,5) -> y=0, x=5
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // dy = 1 (South)
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // dx = 0
            },
            Gene {
                op: OpCode::Fire,
                args: vec![],
            },
        ];

        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;
        vm.context_loc = (0, 0); // Start at 0,0, but migrate to 0,5 (x=0, y=5)
                                 // Wait, migrate takes dy, dx. From 0,0 to 0,5 (x=0, y=5)?
                                 // No, I want Prism at 5,5 (x=5, y=5). Fire from 0,5 (x=0, y=5).
                                 // Migrate dx=0, dy=5.

        // Run setup (Migration + Placement + Fire)
        // Migrate: 1 step. Push(5), Push(0), Migrate.
        // Place Prism: 4 steps. Push(5), Push(5), Push(0), Prism.
        // Fire: 4 steps. Push(1), Push(0), Push(1), Fire.
        // Total ~9 steps.
        for _ in 0..15 {
            vm.step();
        }

        assert_eq!(vm.projectiles.len(), 1);
        let p = &vm.projectiles[0];
        // Check if firing from correct pos x=5
        assert!((p.x - 5.0).abs() < 0.1, "Projectile not at x=5");

        // Run until hit
        for _ in 0..6 {
            crate::vm::nova_ballistics::update_projectiles(&mut vm);
        }

        // Should be 3 projectiles now
        assert_eq!(vm.projectiles.len(), 3, "Prism did not split projectile");
    }
}
