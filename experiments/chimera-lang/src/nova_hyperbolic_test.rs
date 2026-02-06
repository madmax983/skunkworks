#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_gravity_well() {
        // 1. Initialize VM
        // DNA: [ push(6) shape() ] -> Switch to Hyperbolic
        let init_genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(6)] },
            Gene { op: OpCode::Shape, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(init_genes));
        vm.step(); // push
        vm.step(); // shape

        // Verify topology changed
        // We can't access vm.topology if it's not pub or we are not in same module.
        // It is pub.
        if let crate::vm::Topology::Hyperbolic = vm.topology {
            // Good
        } else {
            panic!("Topology did not switch to Hyperbolic");
        }

        // 2. Test Center Radiation
        // DNA: [ push(1) push(5) push(8) push(8) radiate() ]
        // Writes 1 to radius 5 at 8,8
        let center_genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
            Gene { op: OpCode::Radiate, args: vec![] },
        ];
        // Reset grid? Or just use a new VM with same topology
        // Let's manually set topology on a new VM to be clean.
        let mut center_vm = ChimeraVM::new(make_dna(center_genes));
        center_vm.topology = crate::vm::Topology::Hyperbolic;

        while !center_vm.halted {
            center_vm.step();
        }

        let mut center_count = 0;
        for row in &center_vm.grid {
            for cell in row {
                if let Value::Int(1) = cell {
                    center_count += 1;
                }
            }
        }

        // 3. Test Edge Radiation
        // DNA: [ push(1) push(5) push(0) push(0) radiate() ]
        // Writes 1 to radius 5 at 0,0
        let edge_genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Radiate, args: vec![] },
        ];
        let mut edge_vm = ChimeraVM::new(make_dna(edge_genes));
        edge_vm.topology = crate::vm::Topology::Hyperbolic;

        while !edge_vm.halted {
            edge_vm.step();
        }

        let mut edge_count = 0;
        for row in &edge_vm.grid {
            for cell in row {
                if let Value::Int(1) = cell {
                    edge_count += 1;
                }
            }
        }

        println!("Center Count: {}, Edge Count: {}", center_count, edge_count);
        assert!(center_count > edge_count, "Hyperbolic space should have gravity well (center coverage {} > edge coverage {})", center_count, edge_count);
    }
}
