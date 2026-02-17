#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_lisp_eval() {
        let lisp_code = "(+ 10 20)";
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(lisp_code.to_string())],
            },
            Gene {
                op: OpCode::LispEval,
                args: vec![],
            },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step 1: Push string
        vm.step();
        // Step 2: LispEval
        vm.step();

        // Should have 30 on stack
        // Check if stack has 30
        assert!(!vm.stack.is_empty(), "Stack is empty");
        match vm.stack.last().unwrap() {
            Value::Int(30) => (),
            val => panic!("Expected 30, got {:?}", val),
        }
    }

    #[test]
    fn test_lisp_eval_multiple() {
        let lisp_code = "(push 5) (push 3) (+)";
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(lisp_code.to_string())],
            },
            Gene {
                op: OpCode::LispEval,
                args: vec![],
            },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.step(); // push str
        vm.step(); // lisp eval

        // Lisp code: (push 5) (push 3) (+) -> 5 3 + -> 8

        assert!(!vm.stack.is_empty());
        match vm.stack.last().unwrap() {
            Value::Int(8) => (),
            val => panic!("Expected 8, got {:?}", val),
        }
    }
}
