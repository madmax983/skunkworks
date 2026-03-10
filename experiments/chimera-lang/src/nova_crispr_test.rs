#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(strands: Vec<Vec<Gene>>) -> Dna {
        let strands = strands.into_iter().map(|genes| Strand { genes }).collect();
        Dna {
            evolution_config: None,
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
        let target_genes = vec![
            gene("push", Some(1)),
            gene("push", Some(2)),
            gene("add", None),
            gene("push", Some(3)),
        ];

        let guide_genes = vec![gene("push", Some(99)), gene("add", None)];

        let program_genes = vec![
            gene("push", Some(0)),
            gene("push", Some(1)),
            gene("crispr_scan", None),
        ];

        let dna = make_dna(vec![target_genes, guide_genes, program_genes]);
        let mut vm = ChimeraVM::new(dna);
        vm.ip = (2, 0);

        while !vm.halted && vm.ip.0 == 2 {
            vm.step();
        }

        if let Some(Value::Int(idx)) = vm.stack.pop() {
            assert_eq!(idx, 1, "CRISPR Scan should find match at index 1");
        } else {
            panic!("CRISPR Scan failed to return integer index");
        }
    }

    #[test]
    fn test_cas9_cut() {
        let genes = vec![
            gene("push", Some(1)),
            gene("push", Some(2)),
            gene("push", Some(3)),
            gene("push", Some(4)),
        ];

        let program = vec![
            gene("push", Some(0)),
            gene("push", Some(2)),
            gene("cas9_cut", None),
        ];

        let dna = make_dna(vec![genes, program]);
        let mut vm = ChimeraVM::new(dna);
        vm.ip = (1, 0);

        while !vm.halted && vm.ip.0 == 1 {
            vm.step();
        }

        if let Some(Value::Int(idx)) = vm.stack.pop() {
            assert_eq!(idx, 2, "Cas9 Cut should return new strand index 2");
        } else {
            panic!("Cas9 Cut failed to return new strand index");
        }

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
        let s0 = vec![gene("push", Some(1))];
        let s1 = vec![gene("push", Some(2))];
        let program = vec![
            gene("push", Some(0)),
            gene("push", Some(1)),
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

        if let Nucleotide::Number(n) = &vm.dna.helix.strands[0].genes[1].args[0] {
            assert_eq!(*n, 2);
        }
    }
}
