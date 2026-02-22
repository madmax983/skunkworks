#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::vm::{ChimeraVM, Value};
    use crate::opcode::OpCode;

    #[test]
    fn test_lisp_recursion_bomb() {
        // Create a deeply nested Lisp expression
        let depth = 20000;
        let mut lisp_code = String::new();
        for _ in 0..depth {
            lisp_code.push_str("(add ");
        }
        lisp_code.push_str("1 1");
        for _ in 0..depth {
            lisp_code.push(')');
        }

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(lisp_code)],
            },
            Gene {
                op: OpCode::LispEval,
                args: vec![],
            },
        ];

        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // This should crash with a stack overflow
        vm.step(); // Execute Push
        vm.step(); // Execute LispEval
    }
}
