use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_warden_oracle_unwrap() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand {
                genes: vec![
                    Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::String("invalid_gene_string".to_string())],
                    },
                    Gene {
                        op: OpCode::PrologCall,
                        args: vec![],
                    }
                ],
            }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Attempting to parse "invalid_gene_string" as a gene should fail gracefully, not panic
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        vm.step();
    }));
    assert!(result.is_ok(), "VM panicked on invalid oracle prolog call!");
}
