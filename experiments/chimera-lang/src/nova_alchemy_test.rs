#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use crate::vm::nova::OrganelleType;

    fn create_vm() -> ChimeraVM {
        // Create a dummy strand with infinite loop so VM doesn't halt
        let genes = vec![
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }
        ];
        let strand = Strand { genes };
        let dna = Dna {
            helix: Helix { strands: vec![strand] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_alchemy_add() {
        let mut vm = create_vm();

        // Spawn Ribosome
        vm.stack.push(Value::Int(0)); // Strand 0
        vm.stack.push(Value::Int(4)); // Type 4 (Ribosome)
        vm.execute_gene_inner(OpCode::Spawn, &[]);
        vm.step(); // Main runs Jump, Organelle runs 1st tick (8,8->8,9), stack gets garbage

        // Locate Ribosome
        let ribosome_idx = vm.organelles.iter().position(|o| o.kind == OrganelleType::Ribosome).unwrap();

        // Clear Garbage Stack and Setup
        vm.organelles[ribosome_idx].stack.clear();
        vm.organelles[ribosome_idx].stack.push(Value::Int(10));
        vm.organelles[ribosome_idx].stack.push(Value::Int(20));

        // Setup Grid at 8,9 (Where Ribosome is now)
        vm.grid[8][9] = Value::Str("+".to_string());

        vm.step(); // Organelle executes "+"

        // Verify result
        let ribosome = vm.organelles.iter().find(|o| o.kind == OrganelleType::Ribosome).unwrap();
        assert_eq!(ribosome.stack.last(), Some(&Value::Int(30)));
    }

    #[test]
    fn test_alchemy_sub() {
        let mut vm = create_vm();

        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(4));
        vm.execute_gene_inner(OpCode::Spawn, &[]);
        vm.step();

        let ribosome_idx = vm.organelles.iter().position(|o| o.kind == OrganelleType::Ribosome).unwrap();

        vm.organelles[ribosome_idx].stack.clear();
        vm.organelles[ribosome_idx].stack.push(Value::Int(20));
        vm.organelles[ribosome_idx].stack.push(Value::Int(5));

        vm.grid[8][9] = Value::Str("-".to_string());
        vm.step();

        let ribosome = vm.organelles.iter().find(|o| o.kind == OrganelleType::Ribosome).unwrap();
        assert_eq!(ribosome.stack.last(), Some(&Value::Int(15)));
    }

    #[test]
    fn test_alchemy_logic_eq() {
        let mut vm = create_vm();
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(4));
        vm.execute_gene_inner(OpCode::Spawn, &[]);
        vm.step();

        let ribosome_idx = vm.organelles.iter().position(|o| o.kind == OrganelleType::Ribosome).unwrap();

        vm.organelles[ribosome_idx].stack.clear();
        vm.organelles[ribosome_idx].stack.push(Value::Int(5));
        vm.organelles[ribosome_idx].stack.push(Value::Int(5));

        vm.grid[8][9] = Value::Str("=".to_string());
        vm.step();

        let ribosome = vm.organelles.iter().find(|o| o.kind == OrganelleType::Ribosome).unwrap();
        assert_eq!(ribosome.stack.last(), Some(&Value::Int(1)));
    }

    #[test]
    fn test_alchemy_logic_not() {
        let mut vm = create_vm();
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(4));
        vm.execute_gene_inner(OpCode::Spawn, &[]);
        vm.step();

        let ribosome_idx = vm.organelles.iter().position(|o| o.kind == OrganelleType::Ribosome).unwrap();

        vm.organelles[ribosome_idx].stack.clear();
        vm.organelles[ribosome_idx].stack.push(Value::Int(0));

        vm.grid[8][9] = Value::Str("!".to_string());
        vm.step();

        let ribosome = vm.organelles.iter().find(|o| o.kind == OrganelleType::Ribosome).unwrap();
        assert_eq!(ribosome.stack.last(), Some(&Value::Int(1)));
    }

    #[test]
    fn test_alchemy_io_write() {
        let mut vm = create_vm();
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(4));
        vm.execute_gene_inner(OpCode::Spawn, &[]);
        vm.step();

        let ribosome_idx = vm.organelles.iter().position(|o| o.kind == OrganelleType::Ribosome).unwrap();

        vm.organelles[ribosome_idx].stack.clear();
        vm.organelles[ribosome_idx].stack.push(Value::Int(99));
        vm.organelles[ribosome_idx].stack.push(Value::Int(1));
        vm.organelles[ribosome_idx].stack.push(Value::Int(0));

        vm.grid[8][9] = Value::Str(":".to_string());
        vm.step();

        // Relative to 8,9: dy=1 -> 9,9
        assert_eq!(vm.grid[9][9], Value::Int(99));
    }

    #[test]
    fn test_alchemy_io_read() {
        let mut vm = create_vm();
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(4));
        vm.execute_gene_inner(OpCode::Spawn, &[]);
        vm.step();

        // Target at 9,9
        vm.grid[9][9] = Value::Int(777);

        let ribosome_idx = vm.organelles.iter().position(|o| o.kind == OrganelleType::Ribosome).unwrap();

        vm.organelles[ribosome_idx].stack.clear();
        vm.organelles[ribosome_idx].stack.push(Value::Int(1));
        vm.organelles[ribosome_idx].stack.push(Value::Int(0));

        vm.grid[8][9] = Value::Str(";".to_string());
        vm.step();

        let ribosome = vm.organelles.iter().find(|o| o.kind == OrganelleType::Ribosome).unwrap();
        assert_eq!(ribosome.stack.last(), Some(&Value::Int(777)));
    }
}
