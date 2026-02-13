#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova_chimeric;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_chimeric_basic() {
        let mut vm = make_vm();

        let code = "10 20 add";
        vm.stack.push(Value::Str(code.to_string()));

        nova_chimeric::exec_chimeric_op(&mut vm, OpCode::Chimeric, &[]);

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(30));
    }

    #[test]
    fn test_chimeric_def() {
        let mut vm = make_vm();

        // Define a macro
        let code = "def test { 5 5 add }";
        vm.stack.push(Value::Str(code.to_string()));
        nova_chimeric::exec_chimeric_op(&mut vm, OpCode::Chimeric, &[]);

        assert_eq!(vm.dna.helix.strands.len(), 1);
        assert!(vm.dictionary.contains_key("test"));

        let idx = vm.dictionary.get("test").unwrap();
        assert_eq!(vm.dna.helix.strands[*idx].genes.len(), 3);
    }

    #[test]
    fn test_chimeric_call() {
        let mut vm = make_vm();

        // Note: exec_chimeric_op only initiates the call (returns jump target)
        // It does not execute the called strand recursively unless we run the VM loop.
        let code = "def inc { 1 add } 10 inc";
        vm.stack.push(Value::Str(code.to_string()));
        let jump = nova_chimeric::exec_chimeric_op(&mut vm, OpCode::Chimeric, &[]);

        // Stack should have 10
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(10));

        // Should have jumped
        assert!(jump.is_some());

        if let Some(target) = jump {
            vm.ip = target;
            // Execute the macro (push 1, add)
            // step() consumes energy, etc.
            // Strand 0 is "inc" { 1, add }

            // Step 1: push 1
            vm.step();
            // Step 2: add
            vm.step();
        }

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(11));
    }
}
