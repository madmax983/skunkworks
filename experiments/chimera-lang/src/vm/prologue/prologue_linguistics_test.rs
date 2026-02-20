#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType};
    use crate::vm::{ChimeraVM, Value};
    use crate::vm::prologue::exec_prologue_tick;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_quote_rune() {
        let mut vm = make_vm();
        // " H E L L O "
        vm.grid[5][1] = Value::Str("\"".to_string());
        vm.grid[5][2] = Value::Str("H".to_string());
        vm.grid[5][3] = Value::Str("E".to_string());
        vm.grid[5][4] = Value::Str("L".to_string());
        vm.grid[5][5] = Value::Str("L".to_string());
        vm.grid[5][6] = Value::Str("O".to_string());
        vm.grid[5][7] = Value::Str("\"".to_string());

        exec_prologue_tick(&mut vm);

        let sig = &vm.prologue_state.signal_grid[5][7];
        assert_eq!(*sig, Some(Value::Str("HELLO".to_string())));
    }

    #[test]
    fn test_regex_rune() {
        let mut vm = make_vm();
        // N Input: "a+" -> ! (N of ®)
        // ! is at [4][5]. It reads West [4][4].
        vm.grid[4][4] = Value::Str("a+".to_string());
        vm.grid[4][5] = Value::Str("!".to_string());

        // W Input: "baaaab" -> ! (W of ®)
        // ! is at [5][4]. It reads West [5][3].
        vm.grid[5][3] = Value::Str("baaaab".to_string());
        vm.grid[5][4] = Value::Str("!".to_string());

        // Rune
        vm.grid[5][5] = Value::Str("®".to_string());

        exec_prologue_tick(&mut vm);

        let sig = &vm.prologue_state.signal_grid[5][5];
        match sig {
            Some(Value::Junction(JunctionType::Any, items)) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0], Value::Str("aaaa".to_string()));
            }
            _ => panic!("Expected Junction with match, got {:?}", sig),
        }
    }

    #[test]
    fn test_parse_rune() {
        let mut vm = make_vm();
        // North Delim: ","
        // ! at [4][5] reads [4][4]
        vm.grid[4][4] = Value::Str(",".to_string());
        vm.grid[4][5] = Value::Str("!".to_string());

        // West Text: "1,2,3"
        // ! at [5][4] reads [5][3]
        vm.grid[5][3] = Value::Str("1,2,3".to_string());
        vm.grid[5][4] = Value::Str("!".to_string());

        // Rune: ;
        vm.grid[5][5] = Value::Str(";".to_string());

        exec_prologue_tick(&mut vm);

        let sig = &vm.prologue_state.signal_grid[5][5];
        match sig {
            Some(Value::Junction(_, items)) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], Value::Str("1".to_string()));
                assert_eq!(items[1], Value::Str("2".to_string()));
                assert_eq!(items[2], Value::Str("3".to_string()));
            }
            _ => panic!("Expected Junction, got {:?}", sig),
        }
    }

    #[test]
    fn test_concat_rune() {
        let mut vm = make_vm();
        // West: Junction(["A", "B"])
        let list = Value::Junction(JunctionType::Any, vec![Value::Str("A".to_string()), Value::Str("B".to_string())]);
        vm.grid[5][3] = list;
        vm.grid[5][4] = Value::Str("!".to_string());

        // North: "-"
        vm.grid[4][4] = Value::Str("-".to_string());
        vm.grid[4][5] = Value::Str("!".to_string());

        // Rune: ©
        vm.grid[5][5] = Value::Str("©".to_string());

        exec_prologue_tick(&mut vm);

        let sig = &vm.prologue_state.signal_grid[5][5];
        assert_eq!(*sig, Some(Value::Str("A-B".to_string())));
    }

    #[test]
    fn test_stringify_fallback() {
        let mut vm = make_vm();
        // West: 42
        // Rune: " (Quote)
        // No matching quote, so it should stringify

        vm.grid[5][3] = Value::Int(42);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("\"".to_string());

        exec_prologue_tick(&mut vm);

        let sig = &vm.prologue_state.signal_grid[5][5];
        assert_eq!(*sig, Some(Value::Str("42".to_string())));
    }
}
