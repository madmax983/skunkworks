#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::vm::alchemy;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_crucible_elemental() {
        let mut vm = make_vm();

        vm.crucible.add(Value::Str("Fire".to_string()));
        vm.crucible.add(Value::Str("Water".to_string()));

        alchemy::transmute_crucible(&mut vm);

        assert_eq!(vm.crucible.contents.len(), 1);
        if let Value::Str(s) = &vm.crucible.contents[0] {
            assert_eq!(s, "Steam");
        } else {
            panic!("Expected Steam");
        }
    }

    #[test]
    fn test_crucible_splicing() {
        let mut vm = make_vm();

        // Add two strands
        let strand_a = Strand { genes: vec![] };
        let strand_b = Strand { genes: vec![] };
        vm.dna.helix.strands.push(strand_a);
        vm.dna.helix.strands.push(strand_b);
        vm.telomeres = vec![50, 50];

        // Add indices to crucible
        vm.crucible.add(Value::Int(0));
        vm.crucible.add(Value::Int(1));

        alchemy::transmute_crucible(&mut vm);

        // Should have created strand 2
        assert_eq!(vm.dna.helix.strands.len(), 3);
        assert_eq!(vm.crucible.contents.len(), 1);
        if let Value::Int(i) = vm.crucible.contents[0] {
            assert_eq!(i, 2);
        } else {
            panic!("Expected new strand index");
        }
    }

    #[test]
    fn test_crucible_genetic_heat() {
        use crate::ast::{Gene, Nucleotide};
        use crate::opcode::OpCode;

        let mut vm = make_vm();

        // Strand 0: push(5)
        let strand = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }],
        };
        vm.dna.helix.strands.push(strand);
        vm.telomeres = vec![50];

        // Add Strand 0 and "Fire"
        vm.crucible.add(Value::Int(0));
        vm.crucible.add(Value::Str("Fire".to_string()));

        alchemy::transmute_crucible(&mut vm);

        // Should create Strand 1 with push(6)
        assert_eq!(vm.dna.helix.strands.len(), 2);
        let new_strand = &vm.dna.helix.strands[1];
        if let Nucleotide::Number(n) = &new_strand.genes[0].args[0] {
            assert_eq!(*n, 6);
        } else {
            panic!("Expected number arg");
        }
    }
}
