#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};
    use std::fs;

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    fn gene(op: OpCode, args: Vec<Nucleotide>) -> Gene {
        Gene { op, args }
    }

    #[test]
    fn test_akashic_storage() {
        // Because the VM now loads a random file for test environments,
        // we should test it all within one VM instance, or explicitly share the file.
        // Doing it in one VM is safer.
        let genes = vec![
            // Write "foo" -> 42
            gene(OpCode::Push, vec![Nucleotide::String("foo".to_string())]), // Key
            gene(OpCode::Push, vec![Nucleotide::Number(42)]),                // Value
            gene(OpCode::AkashicWrite, vec![]),
            // Read "foo"
            gene(OpCode::Push, vec![Nucleotide::String("foo".to_string())]), // Key
            gene(OpCode::AkashicRead, vec![]),
        ];

        let mut vm = make_vm(genes);
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.pop(), Some(Value::Int(42)));
    }

    #[test]
    fn test_karma_miracle() {
        // Cleanup
        let _ = fs::remove_file(".chimera_akashic.json");

        // 1. Gain Karma
        let karma_genes = vec![
            gene(OpCode::Push, vec![Nucleotide::Number(3000)]),
            gene(OpCode::Karma, vec![]),
        ];
        let mut vm = make_vm(karma_genes);
        // Execute Karma gain
        for _ in 0..2 {
            vm.step();
        }

        // 2. Perform Miracle (Wealth)
        // Wealth Miracle sets energy to high value.
        // Stack: [ MiracleType ] -> Miracle
        // 0=Resurrection, 1=Terraform, 2=Wealth...

        let miracle_genes = vec![
            gene(OpCode::Push, vec![Nucleotide::Number(2)]), // Wealth
            gene(OpCode::Miracle, vec![]),
        ];

        // Hack: Append genes to current strand to continue execution in same VM
        vm.dna.helix.strands[0].genes.extend(miracle_genes);

        // Run Miracle
        for _ in 0..2 {
            vm.step();
        }

        // Wealth should grant massive energy
        assert!(vm.energy > 1000);
    }
}
