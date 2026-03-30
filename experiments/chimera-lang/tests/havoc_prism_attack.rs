#[cfg(feature = "nova")]
#[test]
#[should_panic(expected = "SUCCESS: Unbounded projectile growth detected!")]
fn test_prism_cascade() {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::vm::MAX_PROJECTILES;

    // Strand 0: [ Push(8) Push(5) Salvo ]
    let genes_0 = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(8)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Salvo,
            args: vec![],
        },
    ];
    // Strand 1: [ Jump(1) ] (Infinite Loop to keep VM alive)
    let genes_1 = vec![Gene {
        op: OpCode::Jump,
        args: vec![Nucleotide::Number(1)],
    }];

    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes: genes_0 }, Strand { genes: genes_1 }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.energy = 1_000_000;

    for y in 0..16 {
        for x in 0..16 {
            vm.grid[y][x] = Value::Str("PRISM:0".to_string());
        }
    }

    // Detonate
    for i in 0..=50 {
        vm.step();
        let count = vm.projectiles.len();
        if i % 10 == 0 || i < 10 {
            println!("Tick {}: {} projectiles", i, count);
        }

        // The bug allows projectiles to grow past MAX_PROJECTILES
        // because in `surviving_projectiles.push(p_center);`
        // there is no bounds check. And for left/right branches it's using
        // `surviving_projectiles.len() + vm.projectiles.len() < MAX_PROJECTILES`
        // which doesn't perfectly prevent the count from exceeding the max since
        // `vm.projectiles.len()` is fixed at the start of the tick, but it
        // drains `vm.projectiles`. Wait.
        // `for mut p in std::mem::take(&mut vm.projectiles)`
        // `std::mem::take` sets `vm.projectiles` to empty, so `vm.projectiles.len() == 0`.
        // Thus `surviving_projectiles.len() + 0 < MAX_PROJECTILES` is just
        // `surviving_projectiles.len() < MAX_PROJECTILES`.
        // However, it pushes `p_center` unconditionally, and then pushes `p1` and `p2`
        // if `surviving_projectiles.len() < 1024`.
        // Thus it can reach ~1026 projectiles.
        // It DOES check `< MAX_PROJECTILES`.
        // Wait, why did it reach 3876 projectiles?
        // Ah, `std::mem::take` empties it. The check is:
        // `if surviving_projectiles.len() + vm.projectiles.len() < MAX_PROJECTILES`
        // But `vm.projectiles.len()` is 0 inside the loop!
        // Wait, yes it's 0 inside the loop.
        // So `surviving_projectiles.len() < 1024` allows pushing.
        // But it pushes left and right, then loops to the NEXT projectile.
        // Wait. Next tick.
        // The check `surviving_projectiles.len() + vm.projectiles.len() < MAX_PROJECTILES`
        // is inside `update_projectiles`.
        // During the loop, `surviving_projectiles` grows.
        // The condition will stop pushing NEW left/right splits once `surviving_projectiles.len()` reaches 1024.
        // But it UNCONDITIONALLY pushes `p_center` for EVERY existing projectile hitting a prism.
        // So if we have 1024 projectiles, they all hit a prism.
        // For the first one, `surviving` length is 0. It pushes `center` (len=1), then `left` (len=2), then `right` (len=3).
        // This continues until `surviving_projectiles.len()` reaches 1024.
        // That happens around projectile #341.
        // For the remaining 1024 - 341 = 683 projectiles, the condition `surviving_projectiles.len() < 1024` is FALSE.
        // So it skips pushing `left` and `right`.
        // BUT it STILL UNCONDITIONALLY pushes `p_center`!
        // So it pushes 683 more `p_center`s!
        // Total next tick: 1024 + 683 = 1707 projectiles.
        // Next tick: 1707 projectiles hit a prism.
        // It splits the first 341. `surviving` reaches 1024.
        // It pushes `p_center` for the remaining 1707 - 341 = 1366 projectiles.
        // Total next tick: 1024 + 1366 = 2390 projectiles.
        // Next tick: 2390 projectiles.
        // Splits first 341. Pushes `p_center` for remaining 2390 - 341 = 2049.
        // Total next tick: 1024 + 2049 = 3073 projectiles.
        // It grows linearly by ~683 each tick until they expire!
        // BUT they get TTL=20 from Salvo, +10 if they hit a Lens, etc.
        // If we alternate Prism and Lens, or just use Prisms, TTL will eventually run out.
        // However, we can demonstrate the limit bypass!

        if count > MAX_PROJECTILES * 3 {
            panic!(
                "SUCCESS: Unbounded projectile growth detected! Count: {}",
                count
            );
        }
    }
}
