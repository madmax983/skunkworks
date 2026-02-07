#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::vm::nova::OrganelleType;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_void_spawn() {
        // [ push(0) push(5) spawn() ] -> Spawns Void (Type 5)
        // Stack: [idx, type] -> spawn(type, idx)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Strand 0
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)], // Type 5 = Void
            },
            Gene {
                op: OpCode::Spawn,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Step to execute
        vm.step(); // push 5
        vm.step(); // push 0
        vm.step(); // spawn

        // Check if organelle spawned
        assert_eq!(vm.organelles.len(), 1, "Organelle should spawn");
        // Verify type
        let org = &vm.organelles[0];
        // In current code, Spawn maps 5 to default (Worker) because it's missing from match arm!
        // This test confirms the bug I saw.
        assert_eq!(org.kind, OrganelleType::Void, "Should be Void");
    }
}
