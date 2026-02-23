#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_absorb_geometry_dos() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10000)], // Radius 10,000 -> 400M items -> 6.4GB
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::AbsorbGeometry,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);

        // This should fail gracefully, but currently it will try to allocate 6.4GB
        // We catch the unwind if it panics (OOM might abort though)
        // But in a test environment, OOM usually kills the process.
        // We can't easily catch OOM.
        // But we can check if it returns an error or finishes.

        // If we run this and it crashes, we confirmed the vulnerability.
        // But I don't want to crash the agent.
        // So I'll use a smaller but still illegal radius?
        // MAX_GENES_PER_STRAND is 4096.
        // sqrt(4096) = 64.
        // side = 64. radius = 31.
        // So radius 100 -> side 201 -> count 40,401.
        // This is > 4096 but won't crash the agent.
        // It should still fail logic validation if we implement it correctly.

        // However, to prove "Unbounded Allocation", I need to show it tries to allocate based on input.
        // I will use radius 1000 (4M items, 64MB). Safe for agent, but clearly unbounded.

        // Reset stack to use the pushed values (overriding genes for manual setup ease)
        vm.stack.clear();
        vm.stack.push(Value::Int(1000));
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(0));

        chimera_lang::vm::nova_alchemy_prime::exec_absorb_geometry(
            &mut vm,
            OpCode::AbsorbGeometry,
            &[],
        );

        // Without fix: It allocates 4M items and succeeds (or creates a huge strand).
        // With fix: It should push an error to output and NOT create a strand.

        let last_msg = vm.output.last().cloned().unwrap_or_default();
        if !last_msg.contains("Error") {
            // If no error, check strand length
            let strand_count = vm.dna.helix.strands.len();
            // It started with 1 strand. Should still be 1 if failed, or 2 if success.
            // If success, we have a DoS vector.
            if strand_count > 1 {
                panic!("Vulnerability confirmed: Created oversized strand from radius 1000");
            }
        }
    }
}
