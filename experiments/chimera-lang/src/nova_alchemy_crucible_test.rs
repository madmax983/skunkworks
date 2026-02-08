#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
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
}
