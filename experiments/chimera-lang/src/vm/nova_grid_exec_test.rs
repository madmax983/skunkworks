#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_grid_exec() {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Setup Grid: 5 3 add
        vm.grid[0][0] = Value::Int(5);
        vm.grid[0][1] = Value::Int(3);
        vm.grid[0][2] = Value::Str("add".to_string());

        // Setup Instruction to run GridExec
        // We can manually call the function or run a gene.
        // Let's run via VM gene execution to be integration-y.

        // But since we are inside VM module tests usually, we can call nova::exec_grid_exec directly?
        // No, this is a new file, so we need to access public API or internal if allow.
        // vm.execute_gene_inner is pub(crate).

        let result = vm.execute_gene_inner(OpCode::GridExec, &[]);
        assert_eq!(result, None);

        // Check Stack
        // Should be: [8]
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(8));

        // Check log
        assert!(vm.output.iter().any(|s| s.contains("GRID_EXEC")));
    }

    #[test]
    fn test_grid_exec_flow() {
        // Test that execution follows row-major order
        // [0,0]: 10
        // [0,1]: 20
        // [1,0]: sub (10 - 20 = -10? No, stack is [10, 20]. Sub pops b=20, a=10. a-b = -10. Correct)
        // [1,1]: abs? No abs.
        // [1,1]: dup (stack: [-10, -10])
        // [1,2]: mul (100)

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.grid[0][0] = Value::Int(10);
        vm.grid[0][1] = Value::Int(20);
        vm.grid[1][0] = Value::Str("sub".to_string());
        vm.grid[1][1] = Value::Str("dup".to_string());
        vm.grid[1][2] = Value::Str("mul".to_string());

        vm.execute_gene_inner(OpCode::GridExec, &[]);

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(100));
    }
}
