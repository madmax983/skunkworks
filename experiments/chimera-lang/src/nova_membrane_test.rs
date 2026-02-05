#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_membrane_toggle() {
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // East
            Gene { op: OpCode::Membrane, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (0, 0);

        vm.step(); // push
        vm.step(); // membrane

        // Check (0,0) has East wall (2)
        assert_eq!(vm.membranes[0][0] & ChimeraVM::WALL_E, ChimeraVM::WALL_E);
        // Check (0,1) has West wall (8)
        assert_eq!(vm.membranes[0][1] & ChimeraVM::WALL_W, ChimeraVM::WALL_W);
    }

    #[test]
    fn test_osmosis_blocked() {
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }, // South
            Gene { op: OpCode::Membrane, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }, // South
            Gene { op: OpCode::Osmosis, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (0, 0);

        vm.step(); // push 2
        vm.step(); // membrane (build South wall)
        vm.step(); // push 2
        vm.step(); // osmosis (try move South)

        // Should be blocked
        assert_eq!(vm.context_loc, (0, 0));
        assert!(vm.output.last().unwrap().contains("Blocked"));
    }

    #[test]
    fn test_ribosome_blocked() {
        // Simpler setup: manually configure grid and ribosome
        let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);

        // Wall East at 0,0
        vm.membranes[0][0] |= ChimeraVM::WALL_E;

        // Grid has ">" (Move East)
        vm.grid[0][0] = Value::Str(">".to_string());

        // Add Ribosome at 0,0 moving East (0,1)
        use crate::vm::nova::{Organelle, OrganelleType};
        let ribosome = Organelle {
            stack: vec![],
            ip: (0,0),
            context_loc: (0,0),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Ribosome,
            direction: (0, 1),
        };
        vm.organelles.push(ribosome);

        // Step
        vm.step();

        // Should be at 0,0 still because of wall
        // Note: organelles are processed in step()
        // If it was blocked, context_loc should not change.
        // We need to find the organelle again (step moves it out and back in)
        let org = &vm.organelles[0];
        assert_eq!(org.context_loc, (0, 0));
    }
}
