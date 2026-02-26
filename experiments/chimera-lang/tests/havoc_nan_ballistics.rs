#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use chimera_lang::ast::{Dna, Helix, Gene, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::vm::nova_ballistics::Projectile;

    #[test]
    fn test_nan_teleport_safety() {
        // 👺 HAVOC: Quantum Teleport via NaN
        // EXPECTATION: A NaN projectile should either be discarded or not interact with the grid.
        // REALITY: It teleports to x=0 and destroys the grid cell. This test FAILS if the bug exists.

        // DNA with infinite loop to prevent halting
        let genes = vec![
            Gene { op: OpCode::Jump, args: vec![chimera_lang::ast::Nucleotide::Number(0)] },
        ];

        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![Strand { genes }] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.relativity_mode = true;

        // Inject NaN Projectile
        let p = Projectile {
            x: f64::NAN,
            y: 5.0,
            vx: 0.0,
            vy: 0.0,
            power: 1,
            ttl: 10,
            owner: 0,
            last_hit: None,
        };
        vm.projectiles.push(p);

        // Run step 1 (Bounds check happens here)
        vm.step();

        // Setup collision target at x=0
        vm.grid[5][0] = Value::Int(99);
        println!("Grid[5][0] set to 99");

        // Run step 2 (Collision check happens here)
        vm.step();

        println!("Grid[5][0] is {:?}", vm.grid[5][0]);

        // Assert Safety
        assert_eq!(vm.grid[5][0], Value::Int(99), "TELEPORT ATTACK: Grid cell (5,0) was destroyed by NaN projectile!");
    }
}
