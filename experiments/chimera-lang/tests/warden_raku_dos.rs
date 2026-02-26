#[cfg(test)]
mod tests {
    use chimera_lang::prelude::*;
    use chimera_lang::vm::{ChimeraVM, MAX_JUNCTION_SIZE};

    #[test]
    fn test_cross_product_dos() {
        // Create a VM
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Calculate size that exceeds MAX_JUNCTION_SIZE
        // Sqrt(1024) = 32. So 40x40 = 1600 > 1024.
        let size = 40;
        let mut vec_a = Vec::new();
        let mut vec_b = Vec::new();

        for i in 0..size {
            vec_a.push(Value::Int(i));
            vec_b.push(Value::Int(i));
        }

        let junc_a = Value::Junction(JunctionType::Any, vec_a);
        let junc_b = Value::Junction(JunctionType::Any, vec_b);

        // Push operands: A, B, Operator
        vm.stack.push(junc_a);
        vm.stack.push(junc_b);
        vm.stack.push(Value::Str("+".to_string()));

        // Execute Cross
        // This relies on internal function or we can use exec_raku_op exposed via a shim?
        // Or just inject the OpCode into a strand.
        // Let's execute via OpCode injection for realism.
        vm.dna.helix.strands[0].genes.push(Gene {
            op: OpCode::Cross,
            args: vec![],
        });

        vm.step();

        // Check result
        // If vulnerable, stack has a Junction of size 1600.
        // If secure, stack has Error or nothing pushed (or specific error message).

        if let Some(val) = vm.stack.pop() {
            if let Value::Junction(_, list) = val {
                assert!(
                    list.len() <= MAX_JUNCTION_SIZE,
                    "SECURITY FAILURE: Junction size {} exceeds limit {}",
                    list.len(),
                    MAX_JUNCTION_SIZE
                );
            }
        }
    }
}
