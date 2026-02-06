#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    #[cfg(feature = "nova")]
    fn test_chronostasis() {
        // [ chronostasis(5) push(1) consume() ]
        // We will simulate a scenario where environment would normally decay/diffuse.
        // But with chronostasis, it shouldn't.

        let genes = vec![
            Gene {
                op: OpCode::Chronostasis,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Consume,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        });

        // Setup environment: Add waste at current location
        let (cy, cx) = vm.context_loc;
        vm.waste_grid[cy][cx] = 100;

        // Setup organelle: A Mitochondria that usually generates energy
        // We'll manually inject one since Spawn might not be enough to control timing perfectly
        // But wait, organelles run *after* main step usually.
        // If we freeze, organelles shouldn't run.

        // Let's use waste diffusion as the marker.
        // Normal step diffuses waste.
        // Frozen step should NOT diffuse waste.

        // Step 1: Execute Chronostasis(5)
        vm.step();
        assert_eq!(vm.chronostasis_timer, 5);

        // Initial waste
        let initial_waste = vm.waste_grid[cy][cx];
        assert_eq!(initial_waste, 110); // +10 from process_environment (waaaait)

        // The first step executed Chronostasis.
        // process_environment ran BEFORE the instruction.
        // So waste increased by 10 (metabolism) and diffused?
        // Let's reset for clarity.

        vm.waste_grid[cy][cx] = 100;
        let diff_check = vm.waste_grid[cy][cx];

        // Step 2: Push(10). Should be frozen.
        vm.step();

        // Timer should decrement
        assert_eq!(vm.chronostasis_timer, 4);

        // Waste should NOT have changed (no diffusion, no accumulation)
        // Normal process_environment adds 10 waste and diffuses.
        assert_eq!(vm.waste_grid[cy][cx], diff_check);

        // Run until timer expires
        vm.step(); // 3
        vm.step(); // 2
        vm.step(); // 1
        vm.step(); // 0 - Last frozen tick? Or timer decrements then checks?

        // Let's see implementation details.
        // If timer > 0, skip env. Decrement timer.

        assert_eq!(vm.chronostasis_timer, 0);

        // Step 6: Normal execution resumes
        vm.step();
        // Now waste should change
        assert_ne!(vm.waste_grid[cy][cx], diff_check);
    }
}
