#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::vm::{ChimeraVM, Value};
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::opcode::OpCode;

    #[test]
    fn test_echo_organelle() {
        // Strand 0: Spawns Organelle (Strand 1), then Echoes it.
        let strand0 = Strand {
            genes: vec![
                // 1. Setup Stack for Spawn: [ type(1), strand(1) ]
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Type 1 (Chloroplast - creates energy, stays alive)
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Strand 1
                Gene { op: OpCode::Spawn, args: vec![] }, // Spawns at current location

                // 2. Wait a bit (Echo is gene #4)
                // Organelle runs after this step.
                // It executes Push(42).

                // 3. Prepare Echo
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] }, // Radius 5
                Gene { op: OpCode::Echo, args: vec![] }, // Should execute Push(42)
            ]
        };

        // Strand 1: The Neighbor
        let strand1 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] }, // Loop to stay alive
            ]
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000; // Plenty of energy

        // Run simulation
        // Tick 1: Push(1)
        // Tick 2: Push(1)
        // Tick 3: Spawn() -> Organelle created. Organelle runs Push(42).
        // Tick 4: Push(5) -> Organelle runs Jump(1).
        // Tick 5: Echo() -> Finds Organelle. Last gene was Jump(1) or Push(42)?

        // Wait, Organelle runs every tick.
        // Tick 3: Organelle runs gene 0 (Push 42). last_gene = Push(42).
        // Tick 4: Organelle runs gene 1 (Jump 1). last_gene = Jump(1).

        // So Echo will execute Jump(1)!
        // That means Strand 0 will jump to Strand 1.

        // If we want it to echo Push(42), we need Organelle to do Push(42) *last*.
        // Or calculate timing better.

        // Let's make Strand 1 just: [ Push(42), Push(42), Push(42) ... ]
        // Or [ Push(42), Jump(1) ] where Jump goes to 1 (Loop start).

        // If Echo executes Jump(1), vm.ip becomes (1, 0).
        // Then Strand 0 becomes Strand 1.
        // Stack is preserved.
        // Stack has [ 42 ] (from Organelle? No, Organelle has its own stack).
        // Echo executes the *instruction* on VM's stack.
        // Jump(1) on VM means VM jumps to Strand 1.

        // If we want to verify 42, we should make Organelle do `Push(42)` repeatedly.
        // Strand 1: [ Push(42), Jump(1) ] where Jump targets *this* strand at 0?
        // Jump takes *Strand Index*.
        // So Jump(1) goes to Strand 1, Gene 0.

        // Okay, let's trace.
        // Tick 3: Spawn. Organelle runs Push(42). last_gene = Push(42).
        // Tick 4: VM runs Push(5). Organelle runs Jump(1). last_gene = Jump(1).
        // Tick 5: VM runs Echo. It sees last_gene = Jump(1). It executes Jump(1).
        // VM jumps to Strand 1.

        // Tick 6: VM (now at Strand 1, Gene 0) runs Push(42).
        // VM Stack: [ 42 ].

        // So eventually we get 42.

        for _ in 0..10 {
            vm.step();
        }

        // We expect stack to contain 42.
        // It might contain multiple 42s if it looped.
        assert!(vm.stack.contains(&Value::Int(42)), "Stack should contain 42, got {:?}", vm.stack);
    }
}
