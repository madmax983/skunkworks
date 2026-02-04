#![cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(strands: Vec<Vec<Gene>>) -> Dna {
        let strands = strands.into_iter().map(|genes| Strand { genes }).collect();
        Dna {
            helix: Helix { strands },
        }
    }

    fn gene(name: &str, arg: Option<i64>) -> Gene {
        Gene {
            op: name.parse().unwrap(),
            args: if let Some(n) = arg {
                vec![Nucleotide::Number(n)]
            } else {
                vec![]
            },
        }
    }

    #[test]
    fn test_crispr_scan() {
        // Target: [ push(1), push(2), add(), push(3) ]
        // Guide: [ push(99), add() ] -> We match on NAMES. "push", "add".
        // "push", "add" appears at index 1 in Target ("push(2)", "add()").

        let target_genes = vec![
            gene("push", Some(1)),
            gene("push", Some(2)),
            gene("add", None),
            gene("push", Some(3)),
        ];

        let guide_genes = vec![
            gene("push", Some(99)), // Arg 99 differs from 2, but name "push" matches
            gene("add", None),
        ];

        let program_genes = vec![
            gene("push", Some(0)), // Target strand
            gene("push", Some(1)), // Guide strand
            gene("crispr_scan", None),
        ];

        // Rebuild DNA with 3 strands
        let dna = make_dna(vec![target_genes, guide_genes, program_genes]);
        let mut vm = ChimeraVM::new(dna);

        // Set IP to start of strand 2 (Program)
        vm.ip = (2, 0);

        // Run until program finishes or halts
        while !vm.halted && vm.ip.0 == 2 {
            vm.step();
        }

        // Check stack. Should have found index 1.
        if let Some(Value::Int(idx)) = vm.stack.pop() {
            assert_eq!(idx, 1, "CRISPR Scan should find match at index 1");
        } else {
            panic!("CRISPR Scan failed to return integer index");
        }
    }

    #[test]
    fn test_cas9_cut() {
        // Strand 0: [ push(1), push(2), push(3), push(4) ]
        // Cut at index 2.
        // Result Strand 0: [ push(1), push(2) ]
        // Result Strand 1 (Program)
        // Result Strand 2 (New): [ push(3), push(4) ]

        let genes = vec![
            gene("push", Some(1)),
            gene("push", Some(2)),
            gene("push", Some(3)),
            gene("push", Some(4)),
        ];

        let program = vec![
            gene("push", Some(0)), // Target strand
            gene("push", Some(2)), // Cut index
            gene("cas9_cut", None),
        ];

        let dna = make_dna(vec![genes, program]);
        let mut vm = ChimeraVM::new(dna);
        vm.ip = (1, 0);

        while !vm.halted && vm.ip.0 == 1 {
            vm.step();
        }

        // Stack should have index of new strand (which should be 2, since we had 0 and 1)
        if let Some(Value::Int(idx)) = vm.stack.pop() {
            assert_eq!(idx, 2, "Cas9 Cut should return new strand index 2");
        } else {
            panic!("Cas9 Cut failed to return new strand index");
        }

        // Verify content
        assert_eq!(vm.dna.helix.strands[0].genes.len(), 2);
        assert_eq!(vm.dna.helix.strands[2].genes.len(), 2);
        assert_eq!(vm.dna.helix.strands[2].genes[0].op, OpCode::Push);
        if let Nucleotide::Number(n) = &vm.dna.helix.strands[2].genes[0].args[0] {
            assert_eq!(*n, 3);
        } else {
            panic!("Wrong arg");
        }
    }

    #[test]
    fn test_ligase() {
        // Strand 0: [ push(1) ]
        // Strand 1: [ push(2) ]
        // Ligase(0, 1) -> Strand 0: [ push(1), push(2) ], Strand 1: []

        let s0 = vec![gene("push", Some(1))];
        let s1 = vec![gene("push", Some(2))];
        let program = vec![
            gene("push", Some(0)), // Recipient
            gene("push", Some(1)), // Donor
            gene("ligase", None),
        ];

        let dna = make_dna(vec![s0, s1, program]);
        let mut vm = ChimeraVM::new(dna);
        vm.ip = (2, 0);

        while !vm.halted && vm.ip.0 == 2 {
            vm.step();
        }

        assert_eq!(vm.dna.helix.strands[0].genes.len(), 2);
        assert_eq!(vm.dna.helix.strands[1].genes.len(), 0);

        // Check order
        if let Nucleotide::Number(n) = &vm.dna.helix.strands[0].genes[1].args[0] {
            assert_eq!(*n, 2);
        }
    }
}
