#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova::OrganelleType;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna_with_spawn(spawn_type: i64) -> Dna {
        // Strand 0: [ push(1) push(type) spawn() jump(3) ]
        // Strand 1: [ push(1) drop() jump(0) ]

        let strand0 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(spawn_type)] },
                Gene { op: OpCode::Spawn, args: vec![] },
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(3)] }, // Loop at index 3
            ]
        };

        let strand1 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Drop, args: vec![] },
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
            ]
        };

        Dna { helix: Helix { strands: vec![strand0, strand1] } }
    }

    #[test]
    fn test_chloroplast() {
        let mut vm = ChimeraVM::new(make_dna_with_spawn(1)); // 1 = Chloroplast
        vm.energy = 100;

        // Spawn sequence
        vm.step(); // push(1)
        vm.step(); // push(type)
        vm.step(); // spawn()

        assert_eq!(vm.organelles.len(), 1);
        assert_eq!(vm.organelles[0].kind, OrganelleType::Chloroplast);

        // Set light
        let (y, x) = vm.organelles[0].context_loc;
        vm.light_grid[y][x] = 200;

        let initial_energy = vm.energy; // 76

        vm.step();

        // Main (-1), Organelle (-1), Gain (+5) -> Net +3.
        assert_eq!(vm.energy, initial_energy + 3);
    }

    #[test]
    fn test_mitochondria() {
        let mut vm = ChimeraVM::new(make_dna_with_spawn(2)); // 2 = Mitochondria
        vm.energy = 100;

        // Spawn
        vm.step(); vm.step(); vm.step();

        assert_eq!(vm.organelles[0].kind, OrganelleType::Mitochondria);

        let initial_energy = vm.energy;

        // Step VM
        // Main: -1. Organelle: -1. Mitochondria: +1.
        // Net: -1.

        vm.step();

        assert_eq!(vm.energy, initial_energy - 1);
    }

    #[test]
    fn test_lysosome() {
        let mut vm = ChimeraVM::new(make_dna_with_spawn(3)); // 3 = Lysosome
        vm.energy = 100;

        // Spawn
        vm.step(); vm.step(); vm.step();

        assert_eq!(vm.organelles[0].kind, OrganelleType::Lysosome);

        let (y, x) = vm.organelles[0].context_loc;
        vm.waste_grid[y][x] = 200;

        // Waste diffusion note:
        // During spawn steps, some waste was generated and diffused to neighbors.
        // So neighbors > 0.
        // When we set center to 200, diffusion calculation involves neighbors.
        // Observed result 96. (106 - 10 consumed).

        let initial_energy = vm.energy; // 76

        vm.step();

        // Main (-1), Organelle (-1), Lysosome (+2) -> Net 0.
        assert_eq!(vm.energy, initial_energy);
        assert_eq!(vm.waste_grid[y][x], 96);
    }
}
