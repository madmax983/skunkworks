#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_linguistics_stringify() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit: 42 -> ! -> "
        vm.grid[5][4] = Value::Int(42);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[5][6] = Value::Str("\"".to_string());

        exec_prologue_tick(&mut vm);

        let res = &vm.prologue_state.signal_grid[5][6];
        if let Some(Value::Str(s)) = res {
            assert_eq!(s, "42");
        } else {
            panic!("Expected String '42', got {:?}", res);
        }
    }

    #[test]
    fn test_linguistics_regex() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit:
        //      Pattern -> !
        // Target -> ! -> ®

        vm.grid[5][4] = Value::Str("hello".to_string());
        vm.grid[5][5] = Value::Str("!".to_string());

        // Setup Pattern signal coming from North (4, 6)
        // We need ! at 4,6 reading from 4,5
        vm.grid[4][5] = Value::Str("^h.*o$".to_string());
        vm.grid[4][6] = Value::Str("!".to_string());

        vm.grid[5][6] = Value::Str("®".to_string());

        exec_prologue_tick(&mut vm);

        let res = &vm.prologue_state.signal_grid[5][6];
        if let Some(Value::Int(n)) = res {
            assert_eq!(*n, 1, "Regex should match");
        } else {
            panic!("Expected Int(1), got {:?}", res);
        }
    }

    #[test]
    fn test_linguistics_split() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit:
        //      Delim -> !
        // Target -> ! -> ;

        vm.grid[5][4] = Value::Str("a,b,c".to_string());
        vm.grid[5][5] = Value::Str("!".to_string());

        // Setup Delim signal from North
        vm.grid[4][5] = Value::Str(",".to_string());
        vm.grid[4][6] = Value::Str("!".to_string());

        vm.grid[5][6] = Value::Str(";".to_string());

        exec_prologue_tick(&mut vm);

        let res = &vm.prologue_state.signal_grid[5][6];
        if let Some(Value::Junction(_, list)) = res {
            assert_eq!(list.len(), 3);
            assert_eq!(list[0], Value::Str("a".to_string()));
            assert_eq!(list[1], Value::Str("b".to_string()));
            assert_eq!(list[2], Value::Str("c".to_string()));
        } else {
            panic!("Expected Junction, got {:?}", res);
        }
    }

    #[test]
    fn test_linguistics_join() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit:
        //      Delim -> !
        // List -> ! -> ©

        let list = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("x".to_string()), Value::Str("y".to_string())],
        );
        vm.grid[5][4] = list;
        vm.grid[5][5] = Value::Str("!".to_string());

        // Setup Delim signal from North
        vm.grid[4][5] = Value::Str("-".to_string());
        vm.grid[4][6] = Value::Str("!".to_string());

        vm.grid[5][6] = Value::Str("©".to_string());

        exec_prologue_tick(&mut vm);

        let res = &vm.prologue_state.signal_grid[5][6];
        if let Some(Value::Str(s)) = res {
            assert_eq!(s, "x-y");
        } else {
            panic!("Expected String 'x-y', got {:?}", res);
        }
    }

    #[test]
    fn test_linguistics_shift_case() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit: "hello" -> ! -> ↑
        vm.grid[5][4] = Value::Str("hello".to_string());
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[5][6] = Value::Str("↑".to_string());

        // Circuit: "WORLD" -> ! -> ↓
        vm.grid[7][4] = Value::Str("WORLD".to_string());
        vm.grid[7][5] = Value::Str("!".to_string());
        vm.grid[7][6] = Value::Str("↓".to_string());

        crate::vm::prologue::exec_prologue_tick(&mut vm);

        let res = &vm.prologue_state.signal_grid[5][6];
        if let Some(Value::Str(s)) = res {
            assert_eq!(s, "HELLO");
        } else {
            panic!("Expected String 'HELLO', got {:?}", res);
        }

        let res = &vm.prologue_state.signal_grid[7][6];
        if let Some(Value::Str(s)) = res {
            assert_eq!(s, "world");
        } else {
            panic!("Expected String 'world', got {:?}", res);
        }
    }

    #[test]
    fn test_linguistics_levenshtein() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit:
        //      "sitting" -> ! (North)
        // "kitten" -> ! -> ≅

        vm.grid[5][4] = Value::Str("kitten".to_string());
        vm.grid[5][5] = Value::Str("!".to_string());

        // Setup North signal at (4,6) from (4,5)
        vm.grid[4][5] = Value::Str("sitting".to_string());
        vm.grid[4][6] = Value::Str("!".to_string());

        vm.grid[5][6] = Value::Str("≅".to_string());

        crate::vm::prologue::exec_prologue_tick(&mut vm);

        let res = &vm.prologue_state.signal_grid[5][6];
        if let Some(Value::Int(n)) = res {
            // kitten -> sitting: 3 (k->s, e->i, +g)
            assert_eq!(*n, 3);
        } else {
            panic!("Expected Int(3), got {:?}", res);
        }
    }
}
