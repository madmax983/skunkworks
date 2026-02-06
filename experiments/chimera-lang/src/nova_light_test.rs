#![cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_lumine_shadows() {
        // [ push(5) push(100) lumine() ]
        // Emits light of intensity 100, radius 5.

        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },   // Radius
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] }, // Intensity
            Gene { op: OpCode::Lumine, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.context_loc = (8, 8);

        // Place a wall between (8,8) and (8,9).
        // (8,8) East wall = 4.
        // (8,9) West wall = 8.
        vm.membranes[8][8] |= 4;
        vm.membranes[8][9] |= 8;

        vm.step(); // Execute Push
        vm.step(); // Execute Push
        vm.step(); // Execute Lumine

        // Light at source (8,8) should be 100 (RGB)
        assert_eq!(vm.light_grid[8][8], [100, 100, 100], "Source should be lit");

        // Light at (8,9) should be blocked by the wall
        assert_eq!(vm.light_grid[8][9], [0, 0, 0], "Light should be blocked by wall");

        // Light at (8,7) (West) should be lit (no wall)
        assert_eq!(vm.light_grid[8][7], [100, 100, 100], "Light should reach unblocked cells");
    }
}
