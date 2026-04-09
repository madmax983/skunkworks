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
        // Cleanup
        let _ = fs::remove_file(".chimera_akashic_test_manual.json");

        // 1. Write "foo" -> 42
        let write_genes = vec![
            gene(OpCode::Push, vec![Nucleotide::String("foo".to_string())]), // Key
            gene(OpCode::Push, vec![Nucleotide::Number(42)]),                // Value
            gene(OpCode::AkashicWrite, vec![]),
        ];
        let mut vm_write = make_vm(write_genes);
        // Force the VMs to use the same file for tests to prevent conflicts
        vm_write.akashic.file_path = ".chimera_akashic_test_manual.json".to_string();
        // Step enough to push key, push value, and write
        vm_write.step(); // Push key
        vm_write.step(); // Push val
        vm_write.step(); // AkashicWrite

        // 2. Read "foo"
        let read_genes = vec![
            gene(OpCode::Push, vec![Nucleotide::String("foo".to_string())]),
            gene(OpCode::AkashicRead, vec![]),
        ];
        let mut vm_read = make_vm(read_genes);
        vm_read.akashic.file_path = ".chimera_akashic_test_manual.json".to_string();
        vm_read.akashic = chimera_lang::vm::akashic::AkashicRecords::load_from(".chimera_akashic_test_manual.json").unwrap_or_else(|_| vm_read.akashic);
        vm_read.akashic.file_path = ".chimera_akashic_test_manual.json".to_string();
        vm_read.akashic = chimera_lang::vm::akashic::AkashicRecords::load_from(".chimera_akashic_test_manual.json").unwrap_or_else(|_| vm_read.akashic);
        vm_read.step(); // Push key
        vm_read.step(); // AkashicRead

        assert_eq!(vm_read.stack.pop(), Some(Value::Int(42)));
    }

    #[test]
    fn test_karma_miracle() {
        // Cleanup
        let _ = fs::remove_file(".chimera_akashic_test_manual.json");

        // 1. Gain Karma
        let karma_genes = vec![
            gene(OpCode::Push, vec![Nucleotide::Number(3000)]),
            gene(OpCode::Karma, vec![]),
        ];
        let mut vm = make_vm(karma_genes);
        vm.akashic.file_path = ".chimera_akashic_test_manual2.json".to_string();
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
