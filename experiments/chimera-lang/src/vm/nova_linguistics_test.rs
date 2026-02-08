#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::vm::nova_linguistics;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let genes = vec![];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_levenshtein() {
        let mut vm = make_vm();
        vm.stack.push(Value::Str("kitten".to_string()));
        vm.stack.push(Value::Str("sitting".to_string()));
        nova_linguistics::exec_levenshtein(&mut vm);
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(3));
    }

    #[test]
    fn test_soundex() {
        let mut vm = make_vm();
        // Robert and Rupert have same soundex R163
        vm.stack.push(Value::Str("Robert".to_string()));
        nova_linguistics::exec_soundex(&mut vm);
        let s1 = vm.stack.pop().unwrap();

        vm.stack.push(Value::Str("Rupert".to_string()));
        nova_linguistics::exec_soundex(&mut vm);
        let s2 = vm.stack.pop().unwrap();

        assert_eq!(s1, s2);
        if let Value::Str(s) = s1 {
            assert_eq!(s, "R163");
        }
    }

    #[test]
    fn test_anagram() {
        let mut vm = make_vm();
        vm.stack.push(Value::Str("listen".to_string()));
        vm.stack.push(Value::Str("silent".to_string()));
        nova_linguistics::exec_anagram(&mut vm);
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(1));

        vm.stack.push(Value::Str("cat".to_string()));
        vm.stack.push(Value::Str("dog".to_string()));
        nova_linguistics::exec_anagram(&mut vm);
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(0));
    }

    #[test]
    fn test_cipher() {
        let mut vm = make_vm();
        vm.stack.push(Value::Int(1));
        vm.stack.push(Value::Str("abc".to_string()));
        nova_linguistics::exec_cipher(&mut vm);
        assert_eq!(vm.stack.pop().unwrap(), Value::Str("bcd".to_string()));

        vm.stack.push(Value::Int(1));
        vm.stack.push(Value::Str("Zoo".to_string()));
        nova_linguistics::exec_cipher(&mut vm);
        assert_eq!(vm.stack.pop().unwrap(), Value::Str("App".to_string()));
    }

    #[test]
    fn test_pangram() {
        let mut vm = make_vm();
        vm.stack.push(Value::Str(
            "The quick brown fox jumps over the lazy dog".to_string(),
        ));
        nova_linguistics::exec_pangram(&mut vm);
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(1));

        vm.stack.push(Value::Str("Hello World".to_string()));
        nova_linguistics::exec_pangram(&mut vm);
        assert_eq!(vm.stack.pop().unwrap(), Value::Int(0));
    }
}
