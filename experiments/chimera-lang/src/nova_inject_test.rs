#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use std::fs::File;
    use std::io::Write;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_inject() {
        let filename = "test_inject_payload.dna";
        let payload_content = r#"
            [
                push(777)
                push(888)
                add()
            ]
        "#;

        // Write payload
        let mut file = File::create(filename).expect("Failed to create payload file");
        file.write_all(payload_content.as_bytes())
            .expect("Failed to write payload");

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(filename.to_string())],
            },
            Gene {
                op: OpCode::Inject,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Step 1: Push filename
        vm.step();

        // Step 2: Inject
        vm.step();

        // Cleanup immediately to avoid leftovers if assertion fails later
        let _ = std::fs::remove_file(filename);

        // Check stack has count (1 strand injected)
        assert_eq!(vm.stack.last(), Some(&Value::Int(1)));

        // Check VM has 2 strands now
        assert_eq!(vm.dna.helix.strands.len(), 2);

        // Check telomeres initialized
        assert_eq!(vm.telomeres.len(), 2);
        assert_eq!(vm.telomeres[1], 50);

        // Execute the new strand
        // vm.ip is currently (0, 2) (after Inject).
        // Set ip to (1, 0) manually and run.
        vm.ip = (1, 0);

        vm.step(); // push(777)
        vm.step(); // push(888)
        vm.step(); // add()

        assert_eq!(vm.stack.last(), Some(&Value::Int(1665)));
    }
}
