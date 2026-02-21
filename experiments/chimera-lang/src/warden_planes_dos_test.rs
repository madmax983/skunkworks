#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, MAX_PLANES};

    #[test]
    fn test_planes_unbounded_allocation() {
        // Create a genome that tries to create many planes
        let mut genes = Vec::new();

        // Loop MAX_PLANES + 10 times
        for i in 0..(MAX_PLANES + 10) {
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(i as i64)],
            });
            genes.push(Gene {
                op: OpCode::Dimension,
                args: vec![],
            });
            // Write something to grid to ensure allocation (in case Dimension is lazy)
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            });
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            });
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(99)],
            });
            genes.push(Gene {
                op: OpCode::DWrite,
                args: vec![],
            });
        }

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000000; // Infinite energy

        // Run until halted
        while !vm.halted {
            vm.step();
        }

        // Assert that we have capped the planes
        assert!(
            vm.planes.len() <= MAX_PLANES,
            "Planes count {} exceeded MAX_PLANES {}",
            vm.planes.len(),
            MAX_PLANES
        );

        // Assert that we have an error message
        assert!(
            vm.output
                .iter()
                .any(|s| s.contains("Error: Plane limit exceeded")),
            "Expected error message not found in output: {:?}",
            vm.output
        );
    }
}
